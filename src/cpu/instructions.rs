pub enum Instruction {
    Load(LoadTarget, LoadTarget),
    LoadImmediate(LoadTarget),
    LoadImmediate16(LoadTarget16),
    ReadFromRam(RamAddressRegistry),
    WriteToRamFromStackPointer,
    WriteToRam(RamAddressRegistry),
    Add(ArithmeticTarget),
    AddCarry(ArithmeticTarget),
    Subtract(ArithmeticTarget),
    SubtractCarry(ArithmeticTarget),
    And(ArithmeticTarget),
    Xor(ArithmeticTarget),
    Or(ArithmeticTarget),
    Cp(ArithmeticTarget),
    Add16(ArithmeticTarget16),
    Increment(ArithmeticTarget),
    Decrement(ArithmeticTarget),
    Increment16(ArithmeticTarget16),
    Decrement16(ArithmeticTarget16),
    RotateLeft,
    RotateLeftCarry,
    RotateRight,
    RotateRightCarry,
    DecimalAdjust,
    SetCarryFlag,
    Complement,
    ComplementCarryFlag,
    Jump(JumpCondition),
    JumpHL,
    RelativeJump(JumpCondition),
    Noop,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::Noop),
            0x09 => Some(Instruction::Add16(ArithmeticTarget16::BC)),
            0x19 => Some(Instruction::Add16(ArithmeticTarget16::DE)),
            0x29 => Some(Instruction::Add16(ArithmeticTarget16::HL)),
            0x39 => Some(Instruction::Add16(ArithmeticTarget16::SP)),
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
            0x90 => Some(Instruction::Subtract(ArithmeticTarget::B)),
            0x91 => Some(Instruction::Subtract(ArithmeticTarget::C)),
            0x92 => Some(Instruction::Subtract(ArithmeticTarget::D)),
            0x93 => Some(Instruction::Subtract(ArithmeticTarget::E)),
            0x94 => Some(Instruction::Subtract(ArithmeticTarget::H)),
            0x95 => Some(Instruction::Subtract(ArithmeticTarget::L)),
            0x96 => Some(Instruction::Subtract(ArithmeticTarget::HL)),
            0x97 => Some(Instruction::Subtract(ArithmeticTarget::A)),
            0xd6 => Some(Instruction::Subtract(ArithmeticTarget::Immediate)),
            0x98 => Some(Instruction::SubtractCarry(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SubtractCarry(ArithmeticTarget::C)),
            0x9a => Some(Instruction::SubtractCarry(ArithmeticTarget::D)),
            0x9b => Some(Instruction::SubtractCarry(ArithmeticTarget::E)),
            0x9c => Some(Instruction::SubtractCarry(ArithmeticTarget::H)),
            0x9d => Some(Instruction::SubtractCarry(ArithmeticTarget::L)),
            0x9e => Some(Instruction::SubtractCarry(ArithmeticTarget::HL)),
            0x9f => Some(Instruction::SubtractCarry(ArithmeticTarget::A)),
            0xde => Some(Instruction::SubtractCarry(ArithmeticTarget::Immediate)),
            0xa0 => Some(Instruction::And(ArithmeticTarget::B)),
            0xa1 => Some(Instruction::And(ArithmeticTarget::C)),
            0xa2 => Some(Instruction::And(ArithmeticTarget::D)),
            0xa3 => Some(Instruction::And(ArithmeticTarget::E)),
            0xa4 => Some(Instruction::And(ArithmeticTarget::H)),
            0xa5 => Some(Instruction::And(ArithmeticTarget::L)),
            0xa6 => Some(Instruction::And(ArithmeticTarget::HL)),
            0xa7 => Some(Instruction::And(ArithmeticTarget::A)),
            0xe6 => Some(Instruction::And(ArithmeticTarget::Immediate)),
            0xa8 => Some(Instruction::Xor(ArithmeticTarget::B)),
            0xa9 => Some(Instruction::Xor(ArithmeticTarget::C)),
            0xaa => Some(Instruction::Xor(ArithmeticTarget::D)),
            0xab => Some(Instruction::Xor(ArithmeticTarget::E)),
            0xac => Some(Instruction::Xor(ArithmeticTarget::H)),
            0xad => Some(Instruction::Xor(ArithmeticTarget::L)),
            0xae => Some(Instruction::Xor(ArithmeticTarget::HL)),
            0xaf => Some(Instruction::Xor(ArithmeticTarget::A)),
            0xee => Some(Instruction::Xor(ArithmeticTarget::Immediate)),
            0xb0 => Some(Instruction::Or(ArithmeticTarget::B)),
            0xb1 => Some(Instruction::Or(ArithmeticTarget::C)),
            0xb2 => Some(Instruction::Or(ArithmeticTarget::D)),
            0xb3 => Some(Instruction::Or(ArithmeticTarget::E)),
            0xb4 => Some(Instruction::Or(ArithmeticTarget::H)),
            0xb5 => Some(Instruction::Or(ArithmeticTarget::L)),
            0xb6 => Some(Instruction::Or(ArithmeticTarget::HL)),
            0xb7 => Some(Instruction::Or(ArithmeticTarget::A)),
            0xf6 => Some(Instruction::Or(ArithmeticTarget::Immediate)),
            0xb8 => Some(Instruction::Xor(ArithmeticTarget::B)),
            0xb9 => Some(Instruction::Xor(ArithmeticTarget::C)),
            0xba => Some(Instruction::Xor(ArithmeticTarget::D)),
            0xbb => Some(Instruction::Xor(ArithmeticTarget::E)),
            0xbc => Some(Instruction::Xor(ArithmeticTarget::H)),
            0xbd => Some(Instruction::Xor(ArithmeticTarget::L)),
            0xbe => Some(Instruction::Xor(ArithmeticTarget::HL)),
            0xbf => Some(Instruction::Xor(ArithmeticTarget::A)),
            0xfe => Some(Instruction::Xor(ArithmeticTarget::Immediate)),
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
            0x40 => Some(Instruction::Load(LoadTarget::B, LoadTarget::B)),
            0x41 => Some(Instruction::Load(LoadTarget::B, LoadTarget::C)),
            0x42 => Some(Instruction::Load(LoadTarget::B, LoadTarget::D)),
            0x43 => Some(Instruction::Load(LoadTarget::B, LoadTarget::E)),
            0x44 => Some(Instruction::Load(LoadTarget::B, LoadTarget::H)),
            0x45 => Some(Instruction::Load(LoadTarget::B, LoadTarget::L)),
            0x46 => Some(Instruction::Load(LoadTarget::B, LoadTarget::HL)),
            0x47 => Some(Instruction::Load(LoadTarget::B, LoadTarget::A)),
            0x50 => Some(Instruction::Load(LoadTarget::D, LoadTarget::B)),
            0x51 => Some(Instruction::Load(LoadTarget::D, LoadTarget::C)),
            0x52 => Some(Instruction::Load(LoadTarget::D, LoadTarget::D)),
            0x53 => Some(Instruction::Load(LoadTarget::D, LoadTarget::E)),
            0x54 => Some(Instruction::Load(LoadTarget::D, LoadTarget::H)),
            0x55 => Some(Instruction::Load(LoadTarget::D, LoadTarget::L)),
            0x56 => Some(Instruction::Load(LoadTarget::D, LoadTarget::HL)),
            0x57 => Some(Instruction::Load(LoadTarget::D, LoadTarget::A)),
            0x60 => Some(Instruction::Load(LoadTarget::H, LoadTarget::B)),
            0x61 => Some(Instruction::Load(LoadTarget::H, LoadTarget::C)),
            0x62 => Some(Instruction::Load(LoadTarget::H, LoadTarget::D)),
            0x63 => Some(Instruction::Load(LoadTarget::H, LoadTarget::E)),
            0x64 => Some(Instruction::Load(LoadTarget::H, LoadTarget::H)),
            0x65 => Some(Instruction::Load(LoadTarget::H, LoadTarget::L)),
            0x66 => Some(Instruction::Load(LoadTarget::H, LoadTarget::HL)),
            0x67 => Some(Instruction::Load(LoadTarget::H, LoadTarget::A)),
            0x70 => Some(Instruction::Load(LoadTarget::HL, LoadTarget::B)),
            0x71 => Some(Instruction::Load(LoadTarget::HL, LoadTarget::C)),
            0x72 => Some(Instruction::Load(LoadTarget::HL, LoadTarget::D)),
            0x73 => Some(Instruction::Load(LoadTarget::HL, LoadTarget::E)),
            0x74 => Some(Instruction::Load(LoadTarget::HL, LoadTarget::H)),
            0x75 => Some(Instruction::Load(LoadTarget::HL, LoadTarget::L)),
            0x77 => Some(Instruction::Load(LoadTarget::HL, LoadTarget::A)),
            0x06 => Some(Instruction::LoadImmediate(LoadTarget::B)),
            0x16 => Some(Instruction::LoadImmediate(LoadTarget::D)),
            0x26 => Some(Instruction::LoadImmediate(LoadTarget::H)),
            0x36 => Some(Instruction::LoadImmediate(LoadTarget::HL)),
            0x0e => Some(Instruction::LoadImmediate(LoadTarget::C)),
            0x1e => Some(Instruction::LoadImmediate(LoadTarget::E)),
            0x2e => Some(Instruction::LoadImmediate(LoadTarget::L)),
            0x3e => Some(Instruction::LoadImmediate(LoadTarget::A)),
            0x0a => Some(Instruction::ReadFromRam(RamAddressRegistry::BC)),
            0x1a => Some(Instruction::ReadFromRam(RamAddressRegistry::DE)),
            0x2a => Some(Instruction::ReadFromRam(RamAddressRegistry::HLPlus)),
            0x3a => Some(Instruction::ReadFromRam(RamAddressRegistry::HLMinus)),
            0x02 => Some(Instruction::WriteToRam(RamAddressRegistry::BC)),
            0x12 => Some(Instruction::WriteToRam(RamAddressRegistry::DE)),
            0x22 => Some(Instruction::WriteToRam(RamAddressRegistry::HLPlus)),
            0x32 => Some(Instruction::WriteToRam(RamAddressRegistry::HLMinus)),
            0x08 => Some(Instruction::WriteToRamFromStackPointer),
            0x01 => Some(Instruction::LoadImmediate16(LoadTarget16::BC)),
            0x11 => Some(Instruction::LoadImmediate16(LoadTarget16::DE)),
            0x21 => Some(Instruction::LoadImmediate16(LoadTarget16::HL)),
            0x31 => Some(Instruction::LoadImmediate16(LoadTarget16::SP)),
            0x04 => Some(Instruction::Increment(ArithmeticTarget::B)),
            0x14 => Some(Instruction::Increment(ArithmeticTarget::D)),
            0x24 => Some(Instruction::Increment(ArithmeticTarget::H)),
            0x34 => Some(Instruction::Increment(ArithmeticTarget::HL)),
            0x0c => Some(Instruction::Increment(ArithmeticTarget::C)),
            0x1c => Some(Instruction::Increment(ArithmeticTarget::E)),
            0x2c => Some(Instruction::Increment(ArithmeticTarget::L)),
            0x3c => Some(Instruction::Increment(ArithmeticTarget::A)),
            0x05 => Some(Instruction::Decrement(ArithmeticTarget::B)),
            0x15 => Some(Instruction::Decrement(ArithmeticTarget::D)),
            0x25 => Some(Instruction::Decrement(ArithmeticTarget::H)),
            0x35 => Some(Instruction::Decrement(ArithmeticTarget::HL)),
            0x0d => Some(Instruction::Decrement(ArithmeticTarget::C)),
            0x1d => Some(Instruction::Decrement(ArithmeticTarget::E)),
            0x2d => Some(Instruction::Decrement(ArithmeticTarget::L)),
            0x3d => Some(Instruction::Decrement(ArithmeticTarget::A)),
            0x03 => Some(Instruction::Increment16(ArithmeticTarget16::BC)),
            0x13 => Some(Instruction::Increment16(ArithmeticTarget16::DE)),
            0x23 => Some(Instruction::Increment16(ArithmeticTarget16::HL)),
            0x33 => Some(Instruction::Increment16(ArithmeticTarget16::SP)),
            0x0b => Some(Instruction::Decrement16(ArithmeticTarget16::BC)),
            0x1b => Some(Instruction::Decrement16(ArithmeticTarget16::DE)),
            0x2b => Some(Instruction::Decrement16(ArithmeticTarget16::HL)),
            0x3b => Some(Instruction::Decrement16(ArithmeticTarget16::SP)),
            0x07 => Some(Instruction::RotateLeft),
            0x17 => Some(Instruction::RotateLeftCarry),
            0x0f => Some(Instruction::RotateRight),
            0x1f => Some(Instruction::RotateRightCarry),
            0x27 => Some(Instruction::DecimalAdjust),
            0x37 => Some(Instruction::SetCarryFlag),
            0x2f => Some(Instruction::Complement),
            0x3f => Some(Instruction::ComplementCarryFlag),
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

pub enum ArithmeticTarget16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

pub enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(PartialEq)]
pub enum LoadTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

pub enum RamAddressRegistry {
    BC,
    DE,
    HLPlus,
    HLMinus,
}

pub enum LoadTarget16 {
    BC,
    DE,
    HL,
    SP,
}
