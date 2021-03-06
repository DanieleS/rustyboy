mod instructions;
pub mod interrupts;

use std::fmt::{Display, Formatter};

use self::instructions::{BitOpTarget, ByteArithmeticTarget, LoadTarget16, PushPopTarget};
use crate::memory::Memory;
use crate::utils::int::test_add_carry_bit;
use instructions::{
    ArithmeticTarget, ArithmeticTarget16, Instruction, JumpCondition, LoadTarget,
    MemoryAddressRegistry,
};
use interrupts::{Interrupt, Interrupts};

#[derive(Debug)]
enum ExecutionState {
    Running,
    Halted,
}

#[derive(Debug)]
struct ExecutionStep {
    program_counter: u16,
    cycles: u8,
    state: ExecutionState,
}

impl ExecutionStep {
    fn new(program_counter: u16, cycles: u8) -> ExecutionStep {
        ExecutionStep {
            program_counter,
            cycles,
            state: ExecutionState::Running,
        }
    }

    fn new_with_state(program_counter: u16, cycles: u8, state: ExecutionState) -> ExecutionStep {
        ExecutionStep {
            program_counter,
            cycles,
            state,
        }
    }
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
    pub program_counter: u16,
    pub stack_pointer: u16,
}

impl Registers {
    fn new() -> Self {
        Registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            f: FlagsRegister::from(0xb0),
            h: 0x01,
            l: 0x4d,
            program_counter: 0x100,
            stack_pointer: 0xfffe,
        }
    }

    // 16-bit getter
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (u8::from(&self.f) as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    // 16-bit setter
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = FlagsRegister::from(value as u8);
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "BC={:04X} DE={:04X} HL={:04X} AF={:04X} SP={:04X} PC={:04X}",
            self.get_bc(),
            self.get_de(),
            self.get_hl(),
            self.get_af(),
            self.stack_pointer,
            self.program_counter
        )
    }
}

#[derive(Debug)]
pub struct FlagsRegister {
    pub zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<&FlagsRegister> for u8 {
    fn from(flag: &FlagsRegister) -> Self {
        (flag.zero as u8) << ZERO_FLAG_BYTE_POSITION
            | (flag.subtract as u8) << SUBTRACT_FLAG_BYTE_POSITION
            | (flag.half_carry as u8) << HALF_CARRY_FLAG_BYTE_POSITION
            | (flag.carry as u8) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(value: u8) -> Self {
        FlagsRegister {
            zero: (value >> ZERO_FLAG_BYTE_POSITION) & 0b1 == 1,
            subtract: (value >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1 == 1,
            half_carry: (value >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1 == 1,
            carry: (value >> CARRY_FLAG_BYTE_POSITION) & 0b1 == 1,
        }
    }
}

pub struct Cpu {
    pub registers: Registers,
    pub ime: bool,
    pub halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            ime: false,
            halted: false,
        }
    }

    pub fn step(&mut self, memory_bus: &mut Memory) -> (u8, bool) {
        let mut interrupts = Interrupts::get_interrupts(memory_bus);

        if let Some(ExecutionStep {
            program_counter,
            cycles,
            ..
        }) = execute_interrupts(self, memory_bus, &mut interrupts)
        {
            self.registers.program_counter = program_counter;
            return (cycles, false);
        }

        if self.halted {
            return (1, true);
        }

        let opcode = memory_bus.read(self.registers.program_counter);
        let instruction = Instruction::from_byte(opcode);

        let ExecutionStep {
            program_counter,
            cycles,
            state,
        } = if let Some(instruction) = instruction {
            self.execute(memory_bus, instruction)
        } else {
            panic!("Unknown opcode: {:X}", opcode);
        };

        self.registers.program_counter = program_counter;

        (
            cycles,
            if let ExecutionState::Halted = state {
                true
            } else {
                false
            },
        )
    }

    fn execute(&mut self, memory_bus: &mut Memory, instruction: Instruction) -> ExecutionStep {
        match instruction {
            Instruction::Noop => {
                ExecutionStep::new(self.registers.program_counter.wrapping_add(1), 1)
            }
            Instruction::Load(destination, source) => {
                exeute_load(self, memory_bus, destination, source)
            }
            Instruction::LoadImmediate(destination) => {
                execute_load_immediate(self, memory_bus, destination)
            }
            Instruction::Add(target) => execute_add(self, memory_bus, target),
            Instruction::AddCarry(target) => execute_add_with_carry(self, memory_bus, target),
            Instruction::Subtract(target) => execute_subtract(self, memory_bus, target),
            Instruction::SubtractCarry(target) => {
                execute_subtract_with_carry(self, memory_bus, target)
            }
            Instruction::And(target) => execute_and(self, memory_bus, target),
            Instruction::Xor(target) => execute_xor(self, memory_bus, target),
            Instruction::Or(target) => execute_or(self, memory_bus, target),
            Instruction::Cp(target) => execute_cp(self, memory_bus, target),
            Instruction::Jump(condition) => execute_jump(self, memory_bus, condition),
            Instruction::JumpHL => execute_hl_jump(self),
            Instruction::RelativeJump(condition) => {
                execute_relative_jump(self, memory_bus, condition)
            }
            Instruction::Add16(target) => execute_add16(self, target),
            Instruction::ReadFromRam(address_regisry) => {
                execute_read_from_ram(self, memory_bus, address_regisry)
            }
            Instruction::WriteToRam(address_regisry) => {
                execute_write_to_ram(self, memory_bus, address_regisry)
            }
            Instruction::WriteToRamFromStackPointer => {
                execute_write_to_ram_from_stack_pointer(self, memory_bus)
            }
            Instruction::LoadImmediate16(target) => {
                execute_load_immediate16(self, memory_bus, target)
            }
            Instruction::Increment(target) => execute_increment(self, memory_bus, target),
            Instruction::Decrement(target) => execute_decrement(self, memory_bus, target),
            Instruction::Increment16(target) => execute_increment16(self, target),
            Instruction::Decrement16(target) => execute_decrement16(self, target),
            Instruction::Push(target) => execute_push(self, memory_bus, target),
            Instruction::Pop(target) => execute_pop(self, memory_bus, target),
            Instruction::RotateLeftA => execute_rotate_left_a(self),
            Instruction::RotateLeftCarryA => execute_rotate_left_carry_a(self),
            Instruction::RotateRightA => execute_rotate_right_a(self),
            Instruction::RotateRightCarryA => execute_rotate_right_carry_a(self),
            Instruction::DecimalAdjust => execute_decimal_adjust(self),
            Instruction::SetCarryFlag => execute_set_carry_flag(self),
            Instruction::Complement => execute_complement(self),
            Instruction::ComplementCarryFlag => execute_complement_carry_flag(self),
            Instruction::Stop => execute_stop(self),
            Instruction::DisableInterrupts => execute_disable_interrupts(self),
            Instruction::EnableInterrupts => execute_enable_interrupts(self),
            Instruction::Halt => execute_halt(self),
            Instruction::Call => execute_call(self, memory_bus),
            Instruction::CallCondition(condition) => {
                execute_call_condition(self, memory_bus, condition)
            }
            Instruction::Return => execute_return(self, memory_bus),
            Instruction::ReturnCondition(condition) => {
                execute_return_condition(self, memory_bus, condition)
            }
            Instruction::ReturnAndEnableInterrupts => {
                execute_return_and_enable_interrupts(self, memory_bus)
            }
            Instruction::Restart(address) => execute_restart(self, memory_bus, address),
            Instruction::ExtendedOpcode => execute_extended_opcode(self, memory_bus),
            Instruction::LoadH => execute_load_h(self, memory_bus),
            Instruction::WriteH => execute_write_h(self, memory_bus),
            Instruction::LoadHC => execute_load_hc(self, memory_bus),
            Instruction::WriteHC => execute_write_hc(self, memory_bus),
            Instruction::AddSP => execute_add_sp(self, memory_bus),
            Instruction::LoadSPHL => execute_load_sp_hl(self),
            Instruction::LoadHLSP => execute_load_hl_sp(self, memory_bus),

            //Extended
            Instruction::RotateLeft(target) => execute_rotate_left(self, memory_bus, target),
            Instruction::RotateLeftCarry(target) => {
                execute_rotate_left_carry(self, memory_bus, target)
            }
            Instruction::RotateRight(target) => execute_rotate_right(self, memory_bus, target),
            Instruction::RotateRightCarry(target) => {
                execute_rotate_right_carry(self, memory_bus, target)
            }
            Instruction::ShiftLeftArithmetic(target) => {
                execute_shift_left_arithmetic(self, memory_bus, target)
            }
            Instruction::ShiftRightArithmetic(target) => {
                execute_shift_right_arithmetic(self, memory_bus, target)
            }
            Instruction::Swap(target) => execute_swap(self, memory_bus, target),
            Instruction::ShiftRightLogic(target) => {
                execute_shift_right_logic(self, memory_bus, target)
            }
            Instruction::TestBit(bit_target, target) => {
                execute_test_bit(self, memory_bus, bit_target, target)
            }
            Instruction::ResetBit(bit_target, target) => {
                execute_reset_bit(self, memory_bus, bit_target, target)
            }
            Instruction::SetBit(bit_target, target) => {
                execute_set_bit(self, memory_bus, bit_target, target)
            }
        }
    }

    fn push(&mut self, memory_bus: &mut Memory, value: u16) {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(2);
        memory_bus.write16(self.registers.stack_pointer, value)
    }

    fn pop(&mut self, memory_bus: &mut Memory) -> u16 {
        let value = memory_bus.read16(self.registers.stack_pointer);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(2);
        value
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "Cpu {{")?;
        writeln!(f, "  registers: {},", self.registers)?;
        writeln!(f, "  ime: {}", self.ime)?;
        writeln!(f, "}}")
    }
}

fn get_arictmetic_execution_step(
    program_counter: &u16,
    target: &ArithmeticTarget,
) -> ExecutionStep {
    let pc_steps = match target {
        ArithmeticTarget::Immediate => 2,
        _ => 1,
    };

    let cycles = match target {
        ArithmeticTarget::Immediate => 2,
        ArithmeticTarget::HL => 2,
        _ => 1,
    };
    ExecutionStep::new(program_counter.wrapping_add(pc_steps), cycles)
}

fn execute_arithmetic(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: &ArithmeticTarget,
    function: fn(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep,
) -> ExecutionStep {
    let register_a = cpu.registers.a;
    let register_hl = cpu.registers.get_hl();

    let hl_value = memory_bus.read(register_hl);

    match target {
        ArithmeticTarget::A => function(cpu, target, register_a),
        ArithmeticTarget::B => function(cpu, target, cpu.registers.b),
        ArithmeticTarget::C => function(cpu, target, cpu.registers.c),
        ArithmeticTarget::D => function(cpu, target, cpu.registers.d),
        ArithmeticTarget::E => function(cpu, target, cpu.registers.e),
        ArithmeticTarget::H => function(cpu, target, cpu.registers.h),
        ArithmeticTarget::L => function(cpu, target, cpu.registers.l),
        ArithmeticTarget::HL => function(cpu, target, hl_value),
        ArithmeticTarget::Immediate => {
            let immediate = memory_bus.read(cpu.registers.program_counter + 1);
            function(cpu, target, immediate)
        }
    }
}

fn read_byte_arithmetic_target(
    cpu: &Cpu,
    memory_bus: &Memory,
    target: &ByteArithmeticTarget,
) -> u8 {
    match target {
        ByteArithmeticTarget::A => cpu.registers.a,
        ByteArithmeticTarget::B => cpu.registers.b,
        ByteArithmeticTarget::C => cpu.registers.c,
        ByteArithmeticTarget::D => cpu.registers.d,
        ByteArithmeticTarget::E => cpu.registers.e,
        ByteArithmeticTarget::H => cpu.registers.h,
        ByteArithmeticTarget::L => cpu.registers.l,
        ByteArithmeticTarget::HL => memory_bus.read(cpu.registers.get_hl()),
    }
}

fn write_byte_arithmetic_target(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: &ByteArithmeticTarget,
    value: u8,
) {
    match target {
        ByteArithmeticTarget::A => cpu.registers.a = value,
        ByteArithmeticTarget::B => cpu.registers.b = value,
        ByteArithmeticTarget::C => cpu.registers.c = value,
        ByteArithmeticTarget::D => cpu.registers.d = value,
        ByteArithmeticTarget::E => cpu.registers.e = value,
        ByteArithmeticTarget::H => cpu.registers.h = value,
        ByteArithmeticTarget::L => cpu.registers.l = value,
        ByteArithmeticTarget::HL => memory_bus.write(cpu.registers.get_hl(), value),
    }
}

fn execute_byte_arithmetic(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: &ByteArithmeticTarget,
    function: fn(cpu: &mut Cpu, value: u8) -> u8,
) -> ExecutionStep {
    let value = read_byte_arithmetic_target(cpu, memory_bus, target);
    let value = function(cpu, value);

    write_byte_arithmetic_target(cpu, memory_bus, target, value);

    ExecutionStep::new(
        cpu.registers.program_counter + 1,
        match target {
            ByteArithmeticTarget::HL => 4,
            _ => 2,
        },
    )
}

fn execute_add(
    cpu: &mut Cpu,
    memory_busam: &mut Memory,
    target: ArithmeticTarget,
) -> ExecutionStep {
    fn add(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let (result, overflow) = value.overflowing_add(cpu.registers.a);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = overflow;
        cpu.registers.f.half_carry = (value & 0x0F) + (cpu.registers.a & 0x0F) > 0x0F;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, target)
    }

    execute_arithmetic(cpu, memory_busam, &target, add)
}

fn execute_add_with_carry(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ArithmeticTarget,
) -> ExecutionStep {
    fn add(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let carry = cpu.registers.f.carry as u8;
        let result = value.wrapping_add(cpu.registers.a).wrapping_add(carry);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = cpu.registers.a as u16 + value as u16 + carry as u16 > 0xFF;
        cpu.registers.f.half_carry = (cpu.registers.a & 0xf) + (value & 0xf) + carry > 0xf;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, target)
    }

    execute_arithmetic(cpu, memory_bus, &target, add)
}

fn execute_subtract(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ArithmeticTarget,
) -> ExecutionStep {
    fn subtract(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let (result, overflow) = cpu.registers.a.overflowing_sub(value);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = overflow;
        cpu.registers.f.half_carry =
            (cpu.registers.a & 0xf).wrapping_sub(value & 0xf) & (0xf + 1) != 0;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, target)
    }

    execute_arithmetic(cpu, memory_bus, &target, subtract)
}

fn execute_subtract_with_carry(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ArithmeticTarget,
) -> ExecutionStep {
    fn subtract(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let carry = cpu.registers.f.carry as u8;
        let result = cpu.registers.a.wrapping_sub(value).wrapping_sub(carry);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = (cpu.registers.a as u16) < (value as u16) + (carry as u16);
        cpu.registers.f.half_carry = (cpu.registers.a & 0xf)
            .wrapping_sub(value & 0xf)
            .wrapping_sub(carry)
            & (0xf + 1)
            != 0;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, target)
    }

    execute_arithmetic(cpu, memory_bus, &target, subtract)
}

fn execute_and(cpu: &mut Cpu, memory_bus: &mut Memory, target: ArithmeticTarget) -> ExecutionStep {
    fn and(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let result = value & cpu.registers.a;
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, target)
    }

    execute_arithmetic(cpu, memory_bus, &target, and)
}

fn execute_or(cpu: &mut Cpu, memory_bus: &mut Memory, target: ArithmeticTarget) -> ExecutionStep {
    fn or(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let result = value | cpu.registers.a;
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, target)
    }

    execute_arithmetic(cpu, memory_bus, &target, or)
}

fn execute_xor(cpu: &mut Cpu, memory_bus: &mut Memory, target: ArithmeticTarget) -> ExecutionStep {
    fn xor(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let result = value ^ cpu.registers.a;
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, target)
    }

    execute_arithmetic(cpu, memory_bus, &target, xor)
}

fn execute_cp(cpu: &mut Cpu, memory_bus: &mut Memory, target: ArithmeticTarget) -> ExecutionStep {
    fn cp(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let result = value.wrapping_sub(cpu.registers.a);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = (cpu.registers.a as u16) < (value as u16);
        cpu.registers.f.half_carry =
            (cpu.registers.a & 0x0F).wrapping_sub(value & 0xf) & (0xf + 1) != 0;

        get_arictmetic_execution_step(&cpu.registers.program_counter, target)
    }

    execute_arithmetic(cpu, memory_bus, &target, cp)
}

fn execute_add16(cpu: &mut Cpu, target: ArithmeticTarget16) -> ExecutionStep {
    fn add16(cpu: &mut Cpu, value: u16) -> ExecutionStep {
        let hl = cpu.registers.get_hl();
        let result = hl.wrapping_add(value);
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = hl as u32 + value as u32 > 0xFFFF;
        cpu.registers.f.half_carry = test_add_carry_bit(11, hl, value);

        cpu.registers.set_hl(result);

        ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
    }

    match target {
        ArithmeticTarget16::BC => add16(cpu, cpu.registers.get_bc()),
        ArithmeticTarget16::DE => add16(cpu, cpu.registers.get_de()),
        ArithmeticTarget16::HL => add16(cpu, cpu.registers.get_hl()),
        ArithmeticTarget16::SP => add16(cpu, cpu.registers.stack_pointer),
    }
}

fn check_jump_condition(cpu: &Cpu, condition: JumpCondition) -> bool {
    match condition {
        JumpCondition::Zero => cpu.registers.f.zero,
        JumpCondition::NotZero => !cpu.registers.f.zero,
        JumpCondition::Carry => cpu.registers.f.carry,
        JumpCondition::NotCarry => !cpu.registers.f.carry,
        JumpCondition::Always => true,
    }
}

fn execute_jump(cpu: &mut Cpu, memory_bus: &mut Memory, condition: JumpCondition) -> ExecutionStep {
    let condition_met = check_jump_condition(cpu, condition);

    if condition_met {
        let address = memory_bus.read16(cpu.registers.program_counter + 1);
        ExecutionStep::new(address, 4)
    } else {
        ExecutionStep::new(cpu.registers.program_counter.overflowing_add(3).0, 3)
    }
}

fn execute_hl_jump(cpu: &mut Cpu) -> ExecutionStep {
    ExecutionStep::new(cpu.registers.get_hl(), 1)
}

fn execute_relative_jump(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    condition: JumpCondition,
) -> ExecutionStep {
    let condition_met = check_jump_condition(cpu, condition);

    if condition_met {
        let offset = memory_bus.read_signed(cpu.registers.program_counter + 1);

        let address = cpu.registers.program_counter.wrapping_add(offset as u16);

        ExecutionStep::new(address.wrapping_add(2), 3)
    } else {
        ExecutionStep::new(cpu.registers.program_counter.overflowing_add(2).0, 2)
    }
}

fn exeute_load(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    destination: LoadTarget,
    source: LoadTarget,
) -> ExecutionStep {
    let value = match source {
        LoadTarget::A => cpu.registers.a,
        LoadTarget::B => cpu.registers.b,
        LoadTarget::C => cpu.registers.c,
        LoadTarget::D => cpu.registers.d,
        LoadTarget::E => cpu.registers.e,
        LoadTarget::H => cpu.registers.h,
        LoadTarget::L => cpu.registers.l,
        LoadTarget::HL => memory_bus.read(cpu.registers.get_hl()),
        LoadTarget::ImmediateAddress => {
            let address = memory_bus.read16(cpu.registers.program_counter + 1);
            memory_bus.read(address)
        }
    };

    match destination {
        LoadTarget::A => cpu.registers.a = value,
        LoadTarget::B => cpu.registers.b = value,
        LoadTarget::C => cpu.registers.c = value,
        LoadTarget::D => cpu.registers.d = value,
        LoadTarget::E => cpu.registers.e = value,
        LoadTarget::H => cpu.registers.h = value,
        LoadTarget::L => cpu.registers.l = value,
        LoadTarget::HL => memory_bus.write(cpu.registers.get_hl(), value),
        LoadTarget::ImmediateAddress => {
            let address = memory_bus.read16(cpu.registers.program_counter + 1);
            memory_bus.write(address, value)
        }
    };

    ExecutionStep::new(
        cpu.registers.program_counter.wrapping_add(
            if source == LoadTarget::ImmediateAddress || destination == LoadTarget::ImmediateAddress
            {
                3
            } else {
                1
            },
        ),
        if source == LoadTarget::HL || destination == LoadTarget::HL {
            2
        } else if source == LoadTarget::ImmediateAddress
            || destination == LoadTarget::ImmediateAddress
        {
            4
        } else {
            1
        },
    )
}

fn execute_load_immediate(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    destination: LoadTarget,
) -> ExecutionStep {
    let value = memory_bus.read(cpu.registers.program_counter + 1);

    match destination {
        LoadTarget::A => cpu.registers.a = value,
        LoadTarget::B => cpu.registers.b = value,
        LoadTarget::C => cpu.registers.c = value,
        LoadTarget::D => cpu.registers.d = value,
        LoadTarget::E => cpu.registers.e = value,
        LoadTarget::H => cpu.registers.h = value,
        LoadTarget::L => cpu.registers.l = value,
        LoadTarget::HL => memory_bus.write(cpu.registers.get_hl(), value),
        LoadTarget::ImmediateAddress => (),
    };

    ExecutionStep::new(
        cpu.registers.program_counter.wrapping_add(2),
        match destination {
            LoadTarget::HL => 3,
            _ => 2,
        },
    )
}

fn execute_read_from_ram(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: MemoryAddressRegistry,
) -> ExecutionStep {
    let address = match target {
        MemoryAddressRegistry::BC => cpu.registers.get_bc(),
        MemoryAddressRegistry::DE => cpu.registers.get_de(),
        MemoryAddressRegistry::HLPlus => {
            let hl = cpu.registers.get_hl();
            cpu.registers.set_hl(hl.wrapping_add(1));
            hl
        }
        MemoryAddressRegistry::HLMinus => {
            let hl = cpu.registers.get_hl();
            cpu.registers.set_hl(hl.wrapping_sub(1));
            hl
        }
    };

    let value = memory_bus.read(address);
    cpu.registers.a = value;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
}

fn execute_write_to_ram(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: MemoryAddressRegistry,
) -> ExecutionStep {
    let address = match target {
        MemoryAddressRegistry::BC => cpu.registers.get_bc(),
        MemoryAddressRegistry::DE => cpu.registers.get_de(),
        MemoryAddressRegistry::HLPlus => {
            let hl = cpu.registers.get_hl();
            cpu.registers.set_hl(hl.wrapping_add(1));
            hl
        }
        MemoryAddressRegistry::HLMinus => {
            let hl = cpu.registers.get_hl();
            cpu.registers.set_hl(hl.wrapping_sub(1));
            hl
        }
    };

    let value = cpu.registers.a;
    memory_bus.write(address, value);

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
}

fn execute_write_to_ram_from_stack_pointer(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
) -> ExecutionStep {
    let address = memory_bus.read16(cpu.registers.program_counter.wrapping_add(1));

    memory_bus.write16(address, cpu.registers.stack_pointer);

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(3), 5)
}

fn execute_load_immediate16(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: LoadTarget16,
) -> ExecutionStep {
    let value = memory_bus.read16(cpu.registers.program_counter + 1);

    match target {
        LoadTarget16::BC => cpu.registers.set_bc(value),
        LoadTarget16::DE => cpu.registers.set_de(value),
        LoadTarget16::HL => cpu.registers.set_hl(value),
        LoadTarget16::SP => cpu.registers.stack_pointer = value,
    };

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(3), 3)
}

fn execute_increment(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ArithmeticTarget,
) -> ExecutionStep {
    fn increment(cpu: &mut Cpu, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);

        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = (value & 0xF) == 0xF;
        cpu.registers.f.zero = new_value == 0;

        new_value
    }

    let hl_value = memory_bus.read(cpu.registers.get_hl());

    match target {
        ArithmeticTarget::A => cpu.registers.a = increment(cpu, cpu.registers.a),
        ArithmeticTarget::B => cpu.registers.b = increment(cpu, cpu.registers.b),
        ArithmeticTarget::C => cpu.registers.c = increment(cpu, cpu.registers.c),
        ArithmeticTarget::D => cpu.registers.d = increment(cpu, cpu.registers.d),
        ArithmeticTarget::E => cpu.registers.e = increment(cpu, cpu.registers.e),
        ArithmeticTarget::H => cpu.registers.h = increment(cpu, cpu.registers.h),
        ArithmeticTarget::L => cpu.registers.l = increment(cpu, cpu.registers.l),
        ArithmeticTarget::HL => {
            let new_value = increment(cpu, hl_value);
            memory_bus.write(cpu.registers.get_hl(), new_value)
        }
        ArithmeticTarget::Immediate => (),
    };

    ExecutionStep::new(
        cpu.registers.program_counter.wrapping_add(1),
        match target {
            ArithmeticTarget::HL => 2,
            _ => 1,
        },
    )
}

fn execute_decrement(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ArithmeticTarget,
) -> ExecutionStep {
    fn decrement(cpu: &mut Cpu, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);

        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = (value & 0xF) == 0;
        cpu.registers.f.zero = new_value == 0;

        new_value
    }

    let hl_value = memory_bus.read(cpu.registers.get_hl());

    match target {
        ArithmeticTarget::A => cpu.registers.a = decrement(cpu, cpu.registers.a),
        ArithmeticTarget::B => cpu.registers.b = decrement(cpu, cpu.registers.b),
        ArithmeticTarget::C => cpu.registers.c = decrement(cpu, cpu.registers.c),
        ArithmeticTarget::D => cpu.registers.d = decrement(cpu, cpu.registers.d),
        ArithmeticTarget::E => cpu.registers.e = decrement(cpu, cpu.registers.e),
        ArithmeticTarget::H => cpu.registers.h = decrement(cpu, cpu.registers.h),
        ArithmeticTarget::L => cpu.registers.l = decrement(cpu, cpu.registers.l),
        ArithmeticTarget::HL => {
            let new_value = decrement(cpu, hl_value);
            memory_bus.write(cpu.registers.get_hl(), new_value)
        }
        ArithmeticTarget::Immediate => (),
    };

    ExecutionStep::new(
        cpu.registers.program_counter.wrapping_add(1),
        match target {
            ArithmeticTarget::HL => 2,
            _ => 1,
        },
    )
}

fn execute_increment16(cpu: &mut Cpu, target: ArithmeticTarget16) -> ExecutionStep {
    match target {
        ArithmeticTarget16::BC => cpu.registers.set_bc(cpu.registers.get_bc().wrapping_add(1)),
        ArithmeticTarget16::DE => cpu.registers.set_de(cpu.registers.get_de().wrapping_add(1)),
        ArithmeticTarget16::HL => cpu.registers.set_hl(cpu.registers.get_hl().wrapping_add(1)),
        ArithmeticTarget16::SP => {
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1)
        }
    };

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
}

fn execute_decrement16(cpu: &mut Cpu, target: ArithmeticTarget16) -> ExecutionStep {
    match target {
        ArithmeticTarget16::BC => cpu.registers.set_bc(cpu.registers.get_bc().wrapping_sub(1)),
        ArithmeticTarget16::DE => cpu.registers.set_de(cpu.registers.get_de().wrapping_sub(1)),
        ArithmeticTarget16::HL => cpu.registers.set_hl(cpu.registers.get_hl().wrapping_sub(1)),
        ArithmeticTarget16::SP => {
            cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1)
        }
    };

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
}

fn execute_rotate_left_a(cpu: &mut Cpu) -> ExecutionStep {
    let value = cpu.registers.a;
    let new_value = value.rotate_left(1);

    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x80) != 0;

    cpu.registers.a = new_value;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_rotate_left_carry_a(cpu: &mut Cpu) -> ExecutionStep {
    let value = cpu.registers.a;
    let new_value = (value << 1) | (cpu.registers.f.carry as u8);

    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x80) != 0;

    cpu.registers.a = new_value;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_rotate_right_a(cpu: &mut Cpu) -> ExecutionStep {
    let value = cpu.registers.a;
    let new_value = value.rotate_right(1);

    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x01) != 0;

    cpu.registers.a = new_value;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_rotate_right_carry_a(cpu: &mut Cpu) -> ExecutionStep {
    let value = cpu.registers.a;
    let new_value = (value >> 1) | ((cpu.registers.f.carry as u8) << 7);

    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x01) != 0;

    cpu.registers.a = new_value;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_decimal_adjust(cpu: &mut Cpu) -> ExecutionStep {
    if !cpu.registers.f.subtract {
        if cpu.registers.f.carry || cpu.registers.a > 0x99 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
            cpu.registers.f.carry = true;
        }
        if cpu.registers.f.half_carry || (cpu.registers.a & 0x0f) > 0x09 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x06);
        }
    } else {
        if cpu.registers.f.carry {
            cpu.registers.a = cpu.registers.a.wrapping_sub(0x60);
        }
        if cpu.registers.f.half_carry {
            cpu.registers.a = cpu.registers.a.wrapping_sub(0x06);
        }
    }

    cpu.registers.f.zero = cpu.registers.a == 0;
    cpu.registers.f.half_carry = false;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_set_carry_flag(cpu: &mut Cpu) -> ExecutionStep {
    cpu.registers.f.carry = true;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_complement(cpu: &mut Cpu) -> ExecutionStep {
    cpu.registers.a = !cpu.registers.a;

    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = true;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_complement_carry_flag(cpu: &mut Cpu) -> ExecutionStep {
    cpu.registers.f.carry = !cpu.registers.f.carry;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_stop(cpu: &mut Cpu) -> ! {
    println!("{}", cpu);
    panic!("STOP!");
}

fn execute_disable_interrupts(cpu: &mut Cpu) -> ExecutionStep {
    cpu.ime = false;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_enable_interrupts(cpu: &mut Cpu) -> ExecutionStep {
    cpu.ime = true;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 1)
}

fn execute_halt(cpu: &mut Cpu) -> ExecutionStep {
    ExecutionStep::new_with_state(
        cpu.registers.program_counter.wrapping_add(1),
        1,
        ExecutionState::Halted,
    )
}

fn execute_push(cpu: &mut Cpu, memory_bus: &mut Memory, target: PushPopTarget) -> ExecutionStep {
    let value = match target {
        PushPopTarget::BC => cpu.registers.get_bc(),
        PushPopTarget::DE => cpu.registers.get_de(),
        PushPopTarget::HL => cpu.registers.get_hl(),
        PushPopTarget::AF => cpu.registers.get_af(),
    };

    cpu.push(memory_bus, value);

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 4)
}

fn execute_pop(cpu: &mut Cpu, memory_bus: &mut Memory, target: PushPopTarget) -> ExecutionStep {
    let value = cpu.pop(memory_bus);

    match target {
        PushPopTarget::BC => cpu.registers.set_bc(value),
        PushPopTarget::DE => cpu.registers.set_de(value),
        PushPopTarget::HL => cpu.registers.set_hl(value),
        PushPopTarget::AF => cpu.registers.set_af(value),
    };

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 3)
}

fn execute_call(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    let pc = cpu.registers.program_counter;
    cpu.push(memory_bus, pc.wrapping_add(3));

    let address = memory_bus.read16(cpu.registers.program_counter.wrapping_add(1));

    ExecutionStep::new(address, 6)
}

fn execute_call_condition(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    condition: JumpCondition,
) -> ExecutionStep {
    let condition_met = check_jump_condition(cpu, condition);

    if condition_met {
        execute_call(cpu, memory_bus)
    } else {
        ExecutionStep::new(cpu.registers.program_counter.wrapping_add(3), 3)
    }
}

fn execute_return(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    let address = cpu.pop(memory_bus);
    cpu.registers.program_counter = address;

    ExecutionStep::new(cpu.registers.program_counter, 4)
}

fn execute_return_condition(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    condition: JumpCondition,
) -> ExecutionStep {
    let condition_met = check_jump_condition(cpu, condition);

    if condition_met {
        let pc = execute_return(cpu, memory_bus).program_counter;
        ExecutionStep::new(pc, 4)
    } else {
        ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
    }
}

fn execute_return_and_enable_interrupts(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    execute_enable_interrupts(cpu);
    execute_return(cpu, memory_bus)
}

fn execute_restart(cpu: &mut Cpu, memory_bus: &mut Memory, address: u8) -> ExecutionStep {
    let pc = cpu.registers.program_counter;
    cpu.push(memory_bus, pc.wrapping_add(1));

    ExecutionStep::new(address as u16, 4)
}

fn execute_extended_opcode(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
    let opcode = memory_bus.read(cpu.registers.program_counter);
    let instruction = Instruction::from_byte_extended(opcode);

    cpu.execute(memory_bus, instruction)
}

fn execute_load_h(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    let half_address = memory_bus.read(cpu.registers.program_counter.wrapping_add(1));
    cpu.registers.a = memory_bus.read(half_address as u16 + 0xFF00);

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(2), 3)
}

fn execute_write_h(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    let half_address = memory_bus.read(cpu.registers.program_counter.wrapping_add(1));
    memory_bus.write(half_address as u16 + 0xFF00, cpu.registers.a);

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(2), 3)
}

fn execute_load_hc(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    let half_address = cpu.registers.c;
    cpu.registers.a = memory_bus.read(half_address as u16 + 0xFF00);

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
}

fn execute_write_hc(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    let half_address = cpu.registers.c;
    memory_bus.write(half_address as u16 + 0xFF00, cpu.registers.a);

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
}

fn execute_add_sp(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    let offset =
        memory_bus.read_signed(cpu.registers.program_counter.wrapping_add(1)) as i16 as u16;
    let sp = cpu.registers.stack_pointer;

    cpu.registers.stack_pointer = sp.wrapping_add(offset);
    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = false;
    cpu.registers.f.half_carry = test_add_carry_bit(3, sp, offset);
    cpu.registers.f.carry = test_add_carry_bit(7, sp, offset);

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(2), 4)
}

fn execute_load_sp_hl(cpu: &mut Cpu) -> ExecutionStep {
    let hl = cpu.registers.get_hl();
    cpu.registers.stack_pointer = hl;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
}

fn execute_load_hl_sp(cpu: &mut Cpu, memory_bus: &mut Memory) -> ExecutionStep {
    let sp = cpu.registers.stack_pointer;
    execute_add_sp(cpu, memory_bus);
    cpu.registers.set_hl(cpu.registers.stack_pointer);
    cpu.registers.stack_pointer = sp;

    ExecutionStep::new(cpu.registers.program_counter.wrapping_add(2), 3)
}

fn execute_rotate_left(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    fn rotate_left(cpu: &mut Cpu, value: u8) -> u8 {
        let new_value = value.rotate_left(1);

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = (value & 0x80) != 0;

        new_value
    }

    execute_byte_arithmetic(cpu, memory_bus, &target, rotate_left)
}

fn execute_rotate_left_carry(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    fn rotate_left_carry(cpu: &mut Cpu, value: u8) -> u8 {
        let carry = cpu.registers.f.carry as u8;
        let new_value = value << 1 | carry;

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = (value & 0x80) != 0;

        new_value
    }

    execute_byte_arithmetic(cpu, memory_bus, &target, rotate_left_carry)
}

fn execute_rotate_right(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    fn rotate_right(cpu: &mut Cpu, value: u8) -> u8 {
        let new_value = value.rotate_right(1);

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = (value & 0x01) != 0;

        new_value
    }

    execute_byte_arithmetic(cpu, memory_bus, &target, rotate_right)
}

fn execute_rotate_right_carry(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    fn rotate_left_carry(cpu: &mut Cpu, value: u8) -> u8 {
        let carry = cpu.registers.f.carry as u8;
        let new_value = (value >> 1) | (carry << 7);

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = (value & 0x01) != 0;

        new_value
    }

    execute_byte_arithmetic(cpu, memory_bus, &target, rotate_left_carry)
}

fn execute_shift_left_arithmetic(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    fn shift_left(cpu: &mut Cpu, value: u8) -> u8 {
        let new_value = value << 1;

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = (value & 0x80) != 0;

        new_value
    }

    execute_byte_arithmetic(cpu, memory_bus, &target, shift_left)
}

fn execute_shift_right_arithmetic(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    fn shift_right(cpu: &mut Cpu, value: u8) -> u8 {
        let hi = value & 0x80;
        let new_value = (value >> 1) | hi;

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = (value & 0x01) != 0;

        new_value
    }

    execute_byte_arithmetic(cpu, memory_bus, &target, shift_right)
}

fn execute_swap(
    cpu: &mut Cpu,
    rmemory_busm: &mut Memory,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    fn swap(cpu: &mut Cpu, value: u8) -> u8 {
        let new_value = (value >> 4) | (value << 4);

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = false;

        new_value
    }

    execute_byte_arithmetic(cpu, rmemory_busm, &target, swap)
}

fn execute_shift_right_logic(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    fn shift_right_logic(cpu: &mut Cpu, value: u8) -> u8 {
        let new_value = value >> 1;

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = value & 0x01 != 0;

        new_value
    }

    execute_byte_arithmetic(cpu, memory_bus, &target, shift_right_logic)
}

fn execute_test_bit(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    bit_target: BitOpTarget,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    let value = read_byte_arithmetic_target(cpu, memory_bus, &target);

    let bit_is_zero = match bit_target {
        BitOpTarget::Bit0 => value & 0x01,
        BitOpTarget::Bit1 => value & 0x02,
        BitOpTarget::Bit2 => value & 0x04,
        BitOpTarget::Bit3 => value & 0x08,
        BitOpTarget::Bit4 => value & 0x10,
        BitOpTarget::Bit5 => value & 0x20,
        BitOpTarget::Bit6 => value & 0x40,
        BitOpTarget::Bit7 => value & 0x80,
    } == 0;

    cpu.registers.f.zero = bit_is_zero;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;

    ExecutionStep::new(
        cpu.registers.program_counter.wrapping_add(1),
        match target {
            ByteArithmeticTarget::HL => 4,
            _ => 2,
        },
    )
}

fn execute_reset_bit(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    bit_target: BitOpTarget,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    let value = read_byte_arithmetic_target(cpu, memory_bus, &target);

    let value = match bit_target {
        BitOpTarget::Bit0 => value & 0b1111_1110,
        BitOpTarget::Bit1 => value & 0b1111_1101,
        BitOpTarget::Bit2 => value & 0b1111_1011,
        BitOpTarget::Bit3 => value & 0b1111_0111,
        BitOpTarget::Bit4 => value & 0b1110_1111,
        BitOpTarget::Bit5 => value & 0b1101_1111,
        BitOpTarget::Bit6 => value & 0b1011_1111,
        BitOpTarget::Bit7 => value & 0b0111_1111,
    };

    write_byte_arithmetic_target(cpu, memory_bus, &target, value);

    ExecutionStep::new(
        cpu.registers.program_counter.wrapping_add(1),
        match target {
            ByteArithmeticTarget::HL => 4,
            _ => 2,
        },
    )
}

fn execute_set_bit(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    bit_target: BitOpTarget,
    target: ByteArithmeticTarget,
) -> ExecutionStep {
    let value = read_byte_arithmetic_target(cpu, memory_bus, &target);

    let value = match bit_target {
        BitOpTarget::Bit0 => value | 0b0000_0001,
        BitOpTarget::Bit1 => value | 0b0000_0010,
        BitOpTarget::Bit2 => value | 0b0000_0100,
        BitOpTarget::Bit3 => value | 0b0000_1000,
        BitOpTarget::Bit4 => value | 0b0001_0000,
        BitOpTarget::Bit5 => value | 0b0010_0000,
        BitOpTarget::Bit6 => value | 0b0100_0000,
        BitOpTarget::Bit7 => value | 0b1000_0000,
    };

    write_byte_arithmetic_target(cpu, memory_bus, &target, value);

    ExecutionStep::new(
        cpu.registers.program_counter.wrapping_add(1),
        match target {
            ByteArithmeticTarget::HL => 4,
            _ => 2,
        },
    )
}

fn execute_interrupts(
    cpu: &mut Cpu,
    memory_bus: &mut Memory,
    interrupts: &mut Interrupts,
) -> Option<ExecutionStep> {
    if !cpu.ime {
        cpu.halted = false;
        return None;
    }

    let interrupt = interrupts.get_highest_priority_interrupt();
    let target = if let Some(i) = interrupt {
        Some(match i {
            Interrupt::VBlank => 0x40,
            Interrupt::LcdStat => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60,
        })
    } else {
        None
    };

    target.zip(interrupt).map(|(target, interrupt)| {
        cpu.ime = false;

        cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(2);
        memory_bus.write16(cpu.registers.stack_pointer, cpu.registers.program_counter);

        interrupts.ack_interrupt(interrupt, memory_bus);
        ExecutionStep::new(target, 3)
    })
}
