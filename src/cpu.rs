mod instructions;

use self::instructions::JumpCondition;
use crate::ram::Ram;
use instructions::{ArithmeticTarget, Instruction};

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
    program_counter: u16,
    ram: Ram,
}

impl Cpu {
    fn step(&mut self) -> u8 {
        let opcode = self.ram.read(self.program_counter);
        let instruction = Instruction::from_byte(opcode);
        let ExecutionStep {
            program_counter,
            cycles,
        } = if let Some(instruction) = instruction {
            self.execute(instruction)
        } else {
            panic!("Unknown opcode: {:X}", opcode);
        };

        self.program_counter = program_counter;
        cycles
    }

    fn execute(&mut self, instruction: Instruction) -> ExecutionStep {
        match instruction {
            Instruction::Noop => ExecutionStep::new(self.program_counter.wrapping_add(1), 1),
            Instruction::Add(target) => execute_add(self, target, false),
            Instruction::AddCarry(target) => execute_add(self, target, true),
            Instruction::Subtract(target) => execute_add(self, target, false),
            Instruction::SubtractCarry(target) => execute_add(self, target, true),
            Instruction::And(target) => execute_and(self, target),
            Instruction::Or(target) => execute_or(self, target),
            Instruction::Xor(target) => execute_xor(self, target),
            Instruction::Jump(condition) => execute_jump(self, condition),
            Instruction::JumpHL => execute_hl_jump(self),
            Instruction::RelativeJump(condition) => execute_relative_jump(self, condition),
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
    ExecutionStep::new(program_counter.overflowing_add(pc_steps).0, cycles)
}

fn execute_add(cpu: &mut Cpu, target: ArithmeticTarget, with_carry: bool) -> ExecutionStep {
    let register_a = cpu.registers.a;
    let register_hl = cpu.registers.get_hl();

    let mut add = |value: &u8| {
        let (result, overflow) = if with_carry {
            value.overflowing_add(cpu.registers.a)
        } else {
            let (result, overflow1) = value.overflowing_add(cpu.registers.f.carry as u8);
            let (result, overflow2) = result.overflowing_add(cpu.registers.f.carry as u8);
            (result, overflow1 || overflow2)
        };
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = overflow;
        cpu.registers.f.half_carry = (value & 0x0F) + (cpu.registers.a & 0x0F) > 0x0F;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.program_counter, &target)
    };

    match target {
        ArithmeticTarget::A => add(&register_a),
        ArithmeticTarget::B => add(&cpu.registers.b),
        ArithmeticTarget::C => add(&cpu.registers.c),
        ArithmeticTarget::D => add(&cpu.registers.d),
        ArithmeticTarget::E => add(&cpu.registers.e),
        ArithmeticTarget::H => add(&cpu.registers.h),
        ArithmeticTarget::L => add(&cpu.registers.l),
        ArithmeticTarget::HL => add(&cpu.ram.read(register_hl)),
        ArithmeticTarget::Immediate => {
            let immediate = cpu.ram.read(cpu.registers.program_counter + 1);
            add(&immediate)
        }
    }
}

fn execute_subtract(cpu: &mut Cpu, target: ArithmeticTarget, with_carry: bool) -> ExecutionStep {
    let register_a = cpu.registers.a;
    let register_hl = cpu.registers.get_hl();

    let mut subtract = |value: &u8| {
        let (result, overflow) = if with_carry {
            value.overflowing_sub(cpu.registers.a)
        } else {
            let (result, overflow1) = value.overflowing_sub(cpu.registers.f.carry as u8);
            let (result, overflow2) = result.overflowing_sub(cpu.registers.f.carry as u8);
            (result, overflow1 || overflow2)
        };
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = overflow;
        cpu.registers.f.half_carry = (value & 0x0F) - (cpu.registers.a & 0x0F) > 0x0F;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.program_counter, &target)
    };

    match target {
        ArithmeticTarget::A => subtract(&register_a),
        ArithmeticTarget::B => subtract(&cpu.registers.b),
        ArithmeticTarget::C => subtract(&cpu.registers.c),
        ArithmeticTarget::D => subtract(&cpu.registers.d),
        ArithmeticTarget::E => subtract(&cpu.registers.e),
        ArithmeticTarget::H => subtract(&cpu.registers.h),
        ArithmeticTarget::L => subtract(&cpu.registers.l),
        ArithmeticTarget::HL => subtract(&cpu.ram.read(register_hl)),
        ArithmeticTarget::Immediate => {
            let immediate = cpu.ram.read(cpu.registers.program_counter + 1);
            subtract(&immediate)
        }
    }
}

fn execute_and(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    let register_a = cpu.registers.a;
    let register_hl = cpu.registers.get_hl();

    let mut and = |value: &u8| {
        let result = value & cpu.registers.a;
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.program_counter, &target)
    };

    match target {
        ArithmeticTarget::A => and(&register_a),
        ArithmeticTarget::B => and(&cpu.registers.b),
        ArithmeticTarget::C => and(&cpu.registers.c),
        ArithmeticTarget::D => and(&cpu.registers.d),
        ArithmeticTarget::E => and(&cpu.registers.e),
        ArithmeticTarget::H => and(&cpu.registers.h),
        ArithmeticTarget::L => and(&cpu.registers.l),
        ArithmeticTarget::HL => and(&cpu.ram.read(register_hl)),
        ArithmeticTarget::Immediate => {
            let immediate = cpu.ram.read(cpu.registers.program_counter + 1);
            and(&immediate)
        }
    }
}

fn execute_or(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    let register_a = cpu.registers.a;
    let register_hl = cpu.registers.get_hl();

    let mut or = |value: &u8| {
        let result = value | cpu.registers.a;
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.program_counter, &target)
    };

    match target {
        ArithmeticTarget::A => or(&register_a),
        ArithmeticTarget::B => or(&cpu.registers.b),
        ArithmeticTarget::C => or(&cpu.registers.c),
        ArithmeticTarget::D => or(&cpu.registers.d),
        ArithmeticTarget::E => or(&cpu.registers.e),
        ArithmeticTarget::H => or(&cpu.registers.h),
        ArithmeticTarget::L => or(&cpu.registers.l),
        ArithmeticTarget::HL => or(&cpu.ram.read(register_hl)),
        ArithmeticTarget::Immediate => {
            let immediate = cpu.ram.read(cpu.registers.program_counter + 1);
            or(&immediate)
        }
    }
}

fn execute_xor(cpu: &mut Cpu, target: ArithmeticTarget) -> ExecutionStep {
    let register_a = cpu.registers.a;
    let register_hl = cpu.registers.get_hl();

    let mut xor = |value: &u8| {
        let result = value ^ cpu.registers.a;
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.a = result;

        get_arictmetic_execution_step(&cpu.program_counter, &target)
    };

    match target {
        ArithmeticTarget::A => xor(&register_a),
        ArithmeticTarget::B => xor(&cpu.registers.b),
        ArithmeticTarget::C => xor(&cpu.registers.c),
        ArithmeticTarget::D => xor(&cpu.registers.d),
        ArithmeticTarget::E => xor(&cpu.registers.e),
        ArithmeticTarget::H => xor(&cpu.registers.h),
        ArithmeticTarget::L => xor(&cpu.registers.l),
        ArithmeticTarget::HL => xor(&cpu.ram.read(register_hl)),
        ArithmeticTarget::Immediate => {
            let immediate = cpu.ram.read(cpu.registers.program_counter + 1);
            xor(&immediate)
        }
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
        let address = cpu.ram.read_16(cpu.registers.program_counter + 1);
        ExecutionStep::new(address, 4)
    } else {
        ExecutionStep::new(cpu.program_counter.overflowing_add(3).0, 3)
    }
}

fn execute_hl_jump(cpu: &mut Cpu) -> ExecutionStep {
    ExecutionStep::new(cpu.registers.get_hl(), 1)
}

fn execute_relative_jump(cpu: &mut Cpu, condition: JumpCondition) -> ExecutionStep {
    let condition_met = check_jump_condition(cpu, condition);

    if condition_met {
        let offset = cpu.ram.read_signed(cpu.program_counter + 1);
        let address = cpu.program_counter.overflowing_add(offset as u16).0;
        ExecutionStep::new(address, 3)
    } else {
        ExecutionStep::new(cpu.program_counter.overflowing_add(2).0, 2)
    }
}