pub enum Instruction {
    ADD(ArithmeticTarget),
    ADD_CARRY(ArithmeticTarget),
    JUMP(JumpCondition),
    JUMP_HL,
    RELATIVE_JUMP(JumpCondition),
    NOOP,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOOP),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HL)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0xc6 => Some(Instruction::ADD(ArithmeticTarget::IMMEDIATE)),
            0x88 => Some(Instruction::ADD_CARRY(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADD_CARRY(ArithmeticTarget::C)),
            0x8a => Some(Instruction::ADD_CARRY(ArithmeticTarget::D)),
            0x8b => Some(Instruction::ADD_CARRY(ArithmeticTarget::E)),
            0x8c => Some(Instruction::ADD_CARRY(ArithmeticTarget::H)),
            0x8d => Some(Instruction::ADD_CARRY(ArithmeticTarget::L)),
            0x8e => Some(Instruction::ADD_CARRY(ArithmeticTarget::HL)),
            0x8f => Some(Instruction::ADD_CARRY(ArithmeticTarget::A)),
            0xce => Some(Instruction::ADD_CARRY(ArithmeticTarget::IMMEDIATE)),
            0xc3 => Some(Instruction::JUMP(JumpCondition::Always)),
            0xc2 => Some(Instruction::JUMP(JumpCondition::NotZero)),
            0xd2 => Some(Instruction::JUMP(JumpCondition::NotCarry)),
            0xca => Some(Instruction::JUMP(JumpCondition::Zero)),
            0xda => Some(Instruction::JUMP(JumpCondition::Carry)),
            0xe9 => Some(Instruction::JUMP_HL),
            0x18 => Some(Instruction::RELATIVE_JUMP(JumpCondition::Always)),
            0x20 => Some(Instruction::RELATIVE_JUMP(JumpCondition::NotZero)),
            0x30 => Some(Instruction::RELATIVE_JUMP(JumpCondition::NotCarry)),
            0x28 => Some(Instruction::RELATIVE_JUMP(JumpCondition::Zero)),
            0x38 => Some(Instruction::RELATIVE_JUMP(JumpCondition::Carry)),
            _ => None,
        }
    }
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    IMMEDIATE,
}

pub enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}
