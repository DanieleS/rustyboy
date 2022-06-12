mod instructions;

use crate::ram::Ram;
use crate::utils::int::test_add_carry_bit;
use instructions::{ArithmeticTarget, ArithmeticTarget16, Instruction, JumpCondition, LoadTarget};

#[derive(Debug)]
struct ExecutionStep {
    program_counter: u16,
    cycles: u8,
}

impl ExecutionStep {
    fn new(program_counter: u16, cycles: u8) -> ExecutionStep {
        ExecutionStep {
            program_counter,
            cycles,
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

#[derive(Debug)]
pub struct FlagsRegister {
    zero: bool,
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

struct Cpu {
    registers: Registers,
    ram: Ram,
}

impl Cpu {
    fn step(&mut self) -> u8 {
        let opcode = self.ram.read(self.registers.program_counter);
        let instruction = Instruction::from_byte(opcode);
        let ExecutionStep {
            program_counter,
            cycles,
        } = if let Some(instruction) = instruction {
            self.execute(instruction)
        } else {
            panic!("Unknown opcode: {:X}", opcode);
        };

        self.registers.program_counter = program_counter;
        cycles
    }

    fn execute(&mut self, instruction: Instruction) -> ExecutionStep {
        match instruction {
            Instruction::Noop => {
                ExecutionStep::new(self.registers.program_counter.wrapping_add(1), 1)
            }
            Instruction::Load(destination, source) => exeute_load(self, destination, source),
            Instruction::LoadImmediate(destination) => execute_load_immediate(self, destination),
            Instruction::Add(target) => execute_add(self, target),
            Instruction::AddCarry(target) => execute_add_with_carry(self, target),
            Instruction::Subtract(target) => execute_subtract(self, target),
            Instruction::SubtractCarry(target) => execute_subtract_with_carry(self, target),
            Instruction::And(target) => execute_and(self, target),
            Instruction::Xor(target) => execute_xor(self, target),
            Instruction::Or(target) => execute_or(self, target),
            Instruction::Cp(target) => execute_cp(self, target),
            Instruction::Jump(condition) => execute_jump(self, condition),
            Instruction::JumpHL => execute_hl_jump(self),
            Instruction::RelativeJump(condition) => execute_relative_jump(self, condition),
            Instruction::Add16(target) => execute_add16(self, target),
        }
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
    target: &ArithmeticTarget,
    function: fn(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep,
) -> ExecutionStep {
    let register_a = cpu.registers.a;
    let register_hl = cpu.registers.get_hl();

    match target {
        ArithmeticTarget::A => function(cpu, target, register_a),
        ArithmeticTarget::B => function(cpu, target, cpu.registers.b),
        ArithmeticTarget::C => function(cpu, target, cpu.registers.c),
        ArithmeticTarget::D => function(cpu, target, cpu.registers.d),
        ArithmeticTarget::E => function(cpu, target, cpu.registers.e),
        ArithmeticTarget::H => function(cpu, target, cpu.registers.h),
        ArithmeticTarget::L => function(cpu, target, cpu.registers.l),
        ArithmeticTarget::HL => function(cpu, target, cpu.ram.read(register_hl)),
        ArithmeticTarget::Immediate => {
            let immediate = cpu.ram.read(cpu.registers.program_counter + 1);
            function(cpu, target, immediate)
        }
    }
}

fn execute_add(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    fn add(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let (result, overflow) = value.overflowing_add(cpu.registers.a);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = overflow;
        cpu.registers.f.half_carry = (value & 0x0F) + (cpu.registers.a & 0x0F) > 0x0F;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, &target)
    }

    execute_arithmetic(cpu, &target, add)
}

fn execute_add_with_carry(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    fn add(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let carry = cpu.registers.f.carry as u8;
        let result = value.wrapping_add(cpu.registers.a).wrapping_add(carry);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = cpu.registers.a as u16 + value as u16 + carry as u16 > 0xFF;
        cpu.registers.f.half_carry = (cpu.registers.a & 0xf) + (value & 0xf) + carry > 0xf;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, &target)
    }

    execute_arithmetic(cpu, &target, add)
}

fn execute_subtract(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    fn subtract(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let (result, overflow) = value.overflowing_sub(cpu.registers.a);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = overflow;
        cpu.registers.f.half_carry = (value & 0x0F) - (cpu.registers.a & 0x0F) > 0x0F;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, &target)
    }

    execute_arithmetic(cpu, &target, subtract)
}

fn execute_subtract_with_carry(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    fn subtract(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let carry = cpu.registers.f.carry as u8;
        let result = value.wrapping_sub(cpu.registers.a).wrapping_sub(carry);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = (cpu.registers.a as u16) < (value as u16) + (carry as u16);
        cpu.registers.f.half_carry = (cpu.registers.a & 0xf)
            .wrapping_sub(value & 0xf)
            .wrapping_sub(carry)
            & (0xf + 1)
            != 0;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, &target)
    }

    execute_arithmetic(cpu, &target, subtract)
}

fn execute_and(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    fn and(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let result = value & cpu.registers.a;
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, &target)
    }

    execute_arithmetic(cpu, &target, and)
}

fn execute_or(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    fn or(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let result = value | cpu.registers.a;
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, &target)
    }

    execute_arithmetic(cpu, &target, or)
}

fn execute_xor(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    fn xor(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let result = value ^ cpu.registers.a;
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.registers.program_counter, &target)
    }

    execute_arithmetic(cpu, &target, xor)
}

fn execute_cp(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    fn cp(cpu: &mut Cpu, target: &ArithmeticTarget, value: u8) -> ExecutionStep {
        let (result, overflow) = value.overflowing_sub(cpu.registers.a);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = overflow;
        cpu.registers.f.half_carry = (value & 0x0F) - (cpu.registers.a & 0x0F) > 0x0F;

        get_arictmetic_execution_step(&cpu.registers.program_counter, &target)
    }

    execute_arithmetic(cpu, &target, cp)
}

fn execute_add16(cpu: &mut Cpu, target: ArithmeticTarget16) -> ExecutionStep {
    fn add16(cpu: &mut Cpu, value: u16) -> ExecutionStep {
        let hl = cpu.registers.get_hl();
        let result = hl.wrapping_add(value);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = hl as u32 + value as u32 > 0xFFFF;
        cpu.registers.f.half_carry = test_add_carry_bit(11, hl, value);
        cpu.registers.set_hl(result);

        ExecutionStep::new(cpu.registers.program_counter.wrapping_add(1), 2)
    }

    match target {
        ArithmeticTarget16::AF => add16(cpu, cpu.registers.get_af()),
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

fn execute_jump(cpu: &mut Cpu, condition: JumpCondition) -> ExecutionStep {
    let condition_met = check_jump_condition(cpu, condition);

    if condition_met {
        let address = cpu.ram.read16(cpu.registers.program_counter + 1);
        ExecutionStep::new(address, 4)
    } else {
        ExecutionStep::new(cpu.registers.program_counter.overflowing_add(3).0, 3)
    }
}

fn execute_hl_jump(cpu: &mut Cpu) -> ExecutionStep {
    ExecutionStep::new(cpu.registers.get_hl(), 1)
}

fn execute_relative_jump(cpu: &mut Cpu, condition: JumpCondition) -> ExecutionStep {
    let condition_met = check_jump_condition(cpu, condition);

    if condition_met {
        let offset = cpu.ram.read_signed(cpu.registers.program_counter + 1);
        let address = cpu
            .registers
            .program_counter
            .overflowing_add(offset as u16)
            .0;
        ExecutionStep::new(address, 3)
    } else {
        ExecutionStep::new(cpu.registers.program_counter.overflowing_add(2).0, 2)
    }
}

fn exeute_load(cpu: &mut Cpu, destination: LoadTarget, source: LoadTarget) -> ExecutionStep {
    let value = match source {
        LoadTarget::A => cpu.registers.a,
        LoadTarget::B => cpu.registers.b,
        LoadTarget::C => cpu.registers.c,
        LoadTarget::D => cpu.registers.d,
        LoadTarget::E => cpu.registers.e,
        LoadTarget::H => cpu.registers.h,
        LoadTarget::L => cpu.registers.l,
        LoadTarget::HL => cpu.ram.read(cpu.registers.get_hl()),
    };

    match destination {
        LoadTarget::A => cpu.registers.a = value,
        LoadTarget::B => cpu.registers.b = value,
        LoadTarget::C => cpu.registers.c = value,
        LoadTarget::D => cpu.registers.d = value,
        LoadTarget::E => cpu.registers.e = value,
        LoadTarget::H => cpu.registers.h = value,
        LoadTarget::L => cpu.registers.l = value,
        LoadTarget::HL => cpu.ram.write(cpu.registers.get_hl(), value),
    };

    ExecutionStep {
        program_counter: cpu.registers.program_counter.wrapping_add(1),
        cycles: if source == LoadTarget::HL || destination == LoadTarget::HL {
            2
        } else {
            1
        },
    }
}

fn execute_load_immediate(cpu: &mut Cpu, destination: LoadTarget) -> ExecutionStep {
    let value = cpu.ram.read(cpu.registers.program_counter + 1);

    match destination {
        LoadTarget::A => cpu.registers.a = value,
        LoadTarget::B => cpu.registers.b = value,
        LoadTarget::C => cpu.registers.c = value,
        LoadTarget::D => cpu.registers.d = value,
        LoadTarget::E => cpu.registers.e = value,
        LoadTarget::H => cpu.registers.h = value,
        LoadTarget::L => cpu.registers.l = value,
        LoadTarget::HL => cpu.ram.write(cpu.registers.get_hl(), value),
    };

    ExecutionStep {
        program_counter: cpu.registers.program_counter.wrapping_add(2),
        cycles: if destination == LoadTarget::HL { 3 } else { 2 },
    }
}
