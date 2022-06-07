pub enum Instruction {
    Add(ArithmeticTarget),
    AddCarry(ArithmeticTarget),
    Jump(JumpCondition),
    JumpHL,
    RelativeJump(JumpCondition),
    Noop,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::Noop),
            0x80 => Some(Instruction::Add(ArithmeticTarget::B)),
            0x81 => Some(Instruction::Add(ArithmeticTarget::C)),
            0x82 => Some(Instruction::Add(ArithmeticTarget::D)),
            0x83 => Some(Instruction::Add(ArithmeticTarget::E)),
            0x84 => Some(Instruction::Add(ArithmeticTarget::H)),
            0x85 => Some(Instruction::Add(ArithmeticTarget::L)),
            0x86 => Some(Instruction::Add(ArithmeticTarget::HL)),
            0x87 => Some(Instruction::Add(ArithmeticTarget::A)),
            0xc6 => Some(Instruction::Add(ArithmeticTarget::Immediate)),
            0x88 => Some(Instruction::AddCarry(ArithmeticTarget::B)),
            0x89 => Some(Instruction::AddCarry(ArithmeticTarget::C)),
            0x8a => Some(Instruction::AddCarry(ArithmeticTarget::D)),
            0x8b => Some(Instruction::AddCarry(ArithmeticTarget::E)),
            0x8c => Some(Instruction::AddCarry(ArithmeticTarget::H)),
            0x8d => Some(Instruction::AddCarry(ArithmeticTarget::L)),
            0x8e => Some(Instruction::AddCarry(ArithmeticTarget::HL)),
            0x8f => Some(Instruction::AddCarry(ArithmeticTarget::A)),
            0xce => Some(Instruction::AddCarry(ArithmeticTarget::Immediate)),
            0xc3 => Some(Instruction::Jump(JumpCondition::Always)),
            0xc2 => Some(Instruction::Jump(JumpCondition::NotZero)),
            0xd2 => Some(Instruction::Jump(JumpCondition::NotCarry)),
            0xca => Some(Instruction::Jump(JumpCondition::Zero)),
            0xda => Some(Instruction::Jump(JumpCondition::Carry)),
            0xe9 => Some(Instruction::JumpHL),
            0x18 => Some(Instruction::RelativeJump(JumpCondition::Always)),
            0x20 => Some(Instruction::RelativeJump(JumpCondition::NotZero)),
            0x30 => Some(Instruction::RelativeJump(JumpCondition::NotCarry)),
            0x28 => Some(Instruction::RelativeJump(JumpCondition::Zero)),
            0x38 => Some(Instruction::RelativeJump(JumpCondition::Carry)),
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
    Immediate,
}

pub enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}
