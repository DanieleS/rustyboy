#[derive(Debug)]
pub enum Instruction {
    Load(LoadTarget, LoadTarget),
    LoadImmediate(LoadTarget),
    LoadImmediate16(LoadTarget16),
    LoadSPHL,
    LoadHLSP,
    LoadH,
    WriteH,
    LoadHC,
    WriteHC,
    ReadFromRam(MemoryAddressRegistry),
    WriteToRamFromStackPointer,
    WriteToRam(MemoryAddressRegistry),
    Add(ArithmeticTarget),
    AddCarry(ArithmeticTarget),
    Subtract(ArithmeticTarget),
    SubtractCarry(ArithmeticTarget),
    And(ArithmeticTarget),
    Xor(ArithmeticTarget),
    Or(ArithmeticTarget),
    Cp(ArithmeticTarget),
    Add16(ArithmeticTarget16),
    AddSP,
    Increment(ArithmeticTarget),
    Decrement(ArithmeticTarget),
    Increment16(ArithmeticTarget16),
    Decrement16(ArithmeticTarget16),
    RotateLeftA,
    RotateLeftCarryA,
    RotateRightA,
    RotateRightCarryA,
    DecimalAdjust,
    SetCarryFlag,
    Complement,
    ComplementCarryFlag,
    Jump(JumpCondition),
    JumpHL,
    RelativeJump(JumpCondition),
    Push(PushPopTarget),
    Pop(PushPopTarget),
    Noop,
    Stop,
    DisableInterrupts,
    EnableInterrupts,
    Call,
    CallCondition(JumpCondition),
    Restart(u8),
    Return,
    ReturnCondition(JumpCondition),
    ReturnAndEnableInterrupts,
    Halt,
    ExtendedOpcode,

    // Extended
    RotateLeft(ByteArithmeticTarget),
    RotateRight(ByteArithmeticTarget),
    RotateLeftCarry(ByteArithmeticTarget),
    RotateRightCarry(ByteArithmeticTarget),
    ShiftLeftArithmetic(ByteArithmeticTarget),
    ShiftRightArithmetic(ByteArithmeticTarget),
    Swap(ByteArithmeticTarget),
    ShiftRightLogic(ByteArithmeticTarget),
    TestBit(BitOpTarget, ByteArithmeticTarget),
    ResetBit(BitOpTarget, ByteArithmeticTarget),
    SetBit(BitOpTarget, ByteArithmeticTarget),
}

impl Instruction {
    pub fn from_byte(byte: u8, is_extended_instruction: bool) -> Option<Instruction> {
        if !is_extended_instruction {
            Instruction::from_byte_base(byte)
        } else {
            Instruction::from_byte_extended(byte)
        }
    }

    fn from_byte_base(byte: u8) -> Option<Instruction> {
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
            0xb8 => Some(Instruction::Cp(ArithmeticTarget::B)),
            0xb9 => Some(Instruction::Cp(ArithmeticTarget::C)),
            0xba => Some(Instruction::Cp(ArithmeticTarget::D)),
            0xbb => Some(Instruction::Cp(ArithmeticTarget::E)),
            0xbc => Some(Instruction::Cp(ArithmeticTarget::H)),
            0xbd => Some(Instruction::Cp(ArithmeticTarget::L)),
            0xbe => Some(Instruction::Cp(ArithmeticTarget::HL)),
            0xbf => Some(Instruction::Cp(ArithmeticTarget::A)),
            0xfe => Some(Instruction::Cp(ArithmeticTarget::Immediate)),
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
            0x48 => Some(Instruction::Load(LoadTarget::C, LoadTarget::B)),
            0x49 => Some(Instruction::Load(LoadTarget::C, LoadTarget::C)),
            0x4a => Some(Instruction::Load(LoadTarget::C, LoadTarget::D)),
            0x4b => Some(Instruction::Load(LoadTarget::C, LoadTarget::E)),
            0x4c => Some(Instruction::Load(LoadTarget::C, LoadTarget::H)),
            0x4d => Some(Instruction::Load(LoadTarget::C, LoadTarget::L)),
            0x4e => Some(Instruction::Load(LoadTarget::C, LoadTarget::HL)),
            0x4f => Some(Instruction::Load(LoadTarget::C, LoadTarget::A)),
            0x58 => Some(Instruction::Load(LoadTarget::E, LoadTarget::B)),
            0x59 => Some(Instruction::Load(LoadTarget::E, LoadTarget::C)),
            0x5a => Some(Instruction::Load(LoadTarget::E, LoadTarget::D)),
            0x5b => Some(Instruction::Load(LoadTarget::E, LoadTarget::E)),
            0x5c => Some(Instruction::Load(LoadTarget::E, LoadTarget::H)),
            0x5d => Some(Instruction::Load(LoadTarget::E, LoadTarget::L)),
            0x5e => Some(Instruction::Load(LoadTarget::E, LoadTarget::HL)),
            0x5f => Some(Instruction::Load(LoadTarget::E, LoadTarget::A)),
            0x68 => Some(Instruction::Load(LoadTarget::L, LoadTarget::B)),
            0x69 => Some(Instruction::Load(LoadTarget::L, LoadTarget::C)),
            0x6a => Some(Instruction::Load(LoadTarget::L, LoadTarget::D)),
            0x6b => Some(Instruction::Load(LoadTarget::L, LoadTarget::E)),
            0x6c => Some(Instruction::Load(LoadTarget::L, LoadTarget::H)),
            0x6d => Some(Instruction::Load(LoadTarget::L, LoadTarget::L)),
            0x6e => Some(Instruction::Load(LoadTarget::L, LoadTarget::HL)),
            0x6f => Some(Instruction::Load(LoadTarget::L, LoadTarget::A)),
            0x78 => Some(Instruction::Load(LoadTarget::A, LoadTarget::B)),
            0x79 => Some(Instruction::Load(LoadTarget::A, LoadTarget::C)),
            0x7a => Some(Instruction::Load(LoadTarget::A, LoadTarget::D)),
            0x7b => Some(Instruction::Load(LoadTarget::A, LoadTarget::E)),
            0x7c => Some(Instruction::Load(LoadTarget::A, LoadTarget::H)),
            0x7d => Some(Instruction::Load(LoadTarget::A, LoadTarget::L)),
            0x7e => Some(Instruction::Load(LoadTarget::A, LoadTarget::HL)),
            0x7f => Some(Instruction::Load(LoadTarget::A, LoadTarget::A)),
            0x06 => Some(Instruction::LoadImmediate(LoadTarget::B)),
            0x16 => Some(Instruction::LoadImmediate(LoadTarget::D)),
            0x26 => Some(Instruction::LoadImmediate(LoadTarget::H)),
            0x36 => Some(Instruction::LoadImmediate(LoadTarget::HL)),
            0x0e => Some(Instruction::LoadImmediate(LoadTarget::C)),
            0x1e => Some(Instruction::LoadImmediate(LoadTarget::E)),
            0x2e => Some(Instruction::LoadImmediate(LoadTarget::L)),
            0x3e => Some(Instruction::LoadImmediate(LoadTarget::A)),
            0x0a => Some(Instruction::ReadFromRam(MemoryAddressRegistry::BC)),
            0x1a => Some(Instruction::ReadFromRam(MemoryAddressRegistry::DE)),
            0x2a => Some(Instruction::ReadFromRam(MemoryAddressRegistry::HLPlus)),
            0x3a => Some(Instruction::ReadFromRam(MemoryAddressRegistry::HLMinus)),
            0x02 => Some(Instruction::WriteToRam(MemoryAddressRegistry::BC)),
            0x12 => Some(Instruction::WriteToRam(MemoryAddressRegistry::DE)),
            0x22 => Some(Instruction::WriteToRam(MemoryAddressRegistry::HLPlus)),
            0x32 => Some(Instruction::WriteToRam(MemoryAddressRegistry::HLMinus)),
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
            0x07 => Some(Instruction::RotateLeftA),
            0x17 => Some(Instruction::RotateLeftCarryA),
            0x0f => Some(Instruction::RotateRightA),
            0x1f => Some(Instruction::RotateRightCarryA),
            0x27 => Some(Instruction::DecimalAdjust),
            0x37 => Some(Instruction::SetCarryFlag),
            0x2f => Some(Instruction::Complement),
            0x3f => Some(Instruction::ComplementCarryFlag),
            0x10 => Some(Instruction::Stop),
            0xf3 => Some(Instruction::DisableInterrupts),
            0xfb => Some(Instruction::EnableInterrupts),
            0x76 => Some(Instruction::Halt),
            0xc1 => Some(Instruction::Pop(PushPopTarget::BC)),
            0xd1 => Some(Instruction::Pop(PushPopTarget::DE)),
            0xe1 => Some(Instruction::Pop(PushPopTarget::HL)),
            0xf1 => Some(Instruction::Pop(PushPopTarget::AF)),
            0xc5 => Some(Instruction::Push(PushPopTarget::BC)),
            0xd5 => Some(Instruction::Push(PushPopTarget::DE)),
            0xe5 => Some(Instruction::Push(PushPopTarget::HL)),
            0xf5 => Some(Instruction::Push(PushPopTarget::AF)),
            0xc4 => Some(Instruction::CallCondition(JumpCondition::NotZero)),
            0xcc => Some(Instruction::CallCondition(JumpCondition::Zero)),
            0xd4 => Some(Instruction::CallCondition(JumpCondition::NotCarry)),
            0xdc => Some(Instruction::CallCondition(JumpCondition::Carry)),
            0xcd => Some(Instruction::Call),
            0xc0 => Some(Instruction::ReturnCondition(JumpCondition::NotZero)),
            0xc8 => Some(Instruction::ReturnCondition(JumpCondition::Zero)),
            0xd0 => Some(Instruction::ReturnCondition(JumpCondition::NotCarry)),
            0xd8 => Some(Instruction::ReturnCondition(JumpCondition::Carry)),
            0xc9 => Some(Instruction::Return),
            0xd9 => Some(Instruction::ReturnAndEnableInterrupts),
            0xc7 => Some(Instruction::Restart(0x00)),
            0xd7 => Some(Instruction::Restart(0x10)),
            0xe7 => Some(Instruction::Restart(0x20)),
            0xf7 => Some(Instruction::Restart(0x30)),
            0xcf => Some(Instruction::Restart(0x08)),
            0xdf => Some(Instruction::Restart(0x18)),
            0xef => Some(Instruction::Restart(0x28)),
            0xff => Some(Instruction::Restart(0x38)),
            0xcb => Some(Instruction::ExtendedOpcode),
            0xe0 => Some(Instruction::WriteH),
            0xf0 => Some(Instruction::LoadH),
            0xe2 => Some(Instruction::WriteHC),
            0xf2 => Some(Instruction::LoadHC),
            0xe8 => Some(Instruction::AddSP),
            0xea => Some(Instruction::Load(
                LoadTarget::ImmediateAddress,
                LoadTarget::A,
            )),
            0xfa => Some(Instruction::Load(
                LoadTarget::A,
                LoadTarget::ImmediateAddress,
            )),
            0xf8 => Some(Instruction::LoadSPHL),
            0xf9 => Some(Instruction::LoadHLSP),
            0xd3 => None,
            0xe3 => None,
            0xe4 => None,
            0xf4 => None,
            0xdb => None,
            0xeb => None,
            0xec => None,
            0xfc => None,
            0xdd => None,
            0xed => None,
            0xfd => None,
        }
    }

    fn from_byte_extended(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RotateLeft(ByteArithmeticTarget::B)),
            0x01 => Some(Instruction::RotateLeft(ByteArithmeticTarget::C)),
            0x02 => Some(Instruction::RotateLeft(ByteArithmeticTarget::D)),
            0x03 => Some(Instruction::RotateLeft(ByteArithmeticTarget::E)),
            0x04 => Some(Instruction::RotateLeft(ByteArithmeticTarget::H)),
            0x05 => Some(Instruction::RotateLeft(ByteArithmeticTarget::L)),
            0x06 => Some(Instruction::RotateLeft(ByteArithmeticTarget::HL)),
            0x07 => Some(Instruction::RotateLeft(ByteArithmeticTarget::A)),
            0x08 => Some(Instruction::RotateRight(ByteArithmeticTarget::B)),
            0x09 => Some(Instruction::RotateRight(ByteArithmeticTarget::C)),
            0x0a => Some(Instruction::RotateRight(ByteArithmeticTarget::D)),
            0x0b => Some(Instruction::RotateRight(ByteArithmeticTarget::E)),
            0x0c => Some(Instruction::RotateRight(ByteArithmeticTarget::H)),
            0x0d => Some(Instruction::RotateRight(ByteArithmeticTarget::L)),
            0x0e => Some(Instruction::RotateRight(ByteArithmeticTarget::HL)),
            0x0f => Some(Instruction::RotateRight(ByteArithmeticTarget::A)),
            0x10 => Some(Instruction::RotateLeftCarry(ByteArithmeticTarget::B)),
            0x11 => Some(Instruction::RotateLeftCarry(ByteArithmeticTarget::C)),
            0x12 => Some(Instruction::RotateLeftCarry(ByteArithmeticTarget::D)),
            0x13 => Some(Instruction::RotateLeftCarry(ByteArithmeticTarget::E)),
            0x14 => Some(Instruction::RotateLeftCarry(ByteArithmeticTarget::H)),
            0x15 => Some(Instruction::RotateLeftCarry(ByteArithmeticTarget::L)),
            0x16 => Some(Instruction::RotateLeftCarry(ByteArithmeticTarget::HL)),
            0x17 => Some(Instruction::RotateLeftCarry(ByteArithmeticTarget::A)),
            0x18 => Some(Instruction::RotateRightCarry(ByteArithmeticTarget::B)),
            0x19 => Some(Instruction::RotateRightCarry(ByteArithmeticTarget::C)),
            0x1a => Some(Instruction::RotateRightCarry(ByteArithmeticTarget::D)),
            0x1b => Some(Instruction::RotateRightCarry(ByteArithmeticTarget::E)),
            0x1c => Some(Instruction::RotateRightCarry(ByteArithmeticTarget::H)),
            0x1d => Some(Instruction::RotateRightCarry(ByteArithmeticTarget::L)),
            0x1e => Some(Instruction::RotateRightCarry(ByteArithmeticTarget::HL)),
            0x1f => Some(Instruction::RotateRightCarry(ByteArithmeticTarget::A)),
            0x20 => Some(Instruction::ShiftLeftArithmetic(ByteArithmeticTarget::B)),
            0x21 => Some(Instruction::ShiftLeftArithmetic(ByteArithmeticTarget::C)),
            0x22 => Some(Instruction::ShiftLeftArithmetic(ByteArithmeticTarget::D)),
            0x23 => Some(Instruction::ShiftLeftArithmetic(ByteArithmeticTarget::E)),
            0x24 => Some(Instruction::ShiftLeftArithmetic(ByteArithmeticTarget::H)),
            0x25 => Some(Instruction::ShiftLeftArithmetic(ByteArithmeticTarget::L)),
            0x26 => Some(Instruction::ShiftLeftArithmetic(ByteArithmeticTarget::HL)),
            0x27 => Some(Instruction::ShiftLeftArithmetic(ByteArithmeticTarget::A)),
            0x28 => Some(Instruction::ShiftRightArithmetic(ByteArithmeticTarget::B)),
            0x29 => Some(Instruction::ShiftRightArithmetic(ByteArithmeticTarget::C)),
            0x2a => Some(Instruction::ShiftRightArithmetic(ByteArithmeticTarget::D)),
            0x2b => Some(Instruction::ShiftRightArithmetic(ByteArithmeticTarget::E)),
            0x2c => Some(Instruction::ShiftRightArithmetic(ByteArithmeticTarget::H)),
            0x2d => Some(Instruction::ShiftRightArithmetic(ByteArithmeticTarget::L)),
            0x2e => Some(Instruction::ShiftRightArithmetic(ByteArithmeticTarget::HL)),
            0x2f => Some(Instruction::ShiftRightArithmetic(ByteArithmeticTarget::A)),
            0x30 => Some(Instruction::Swap(ByteArithmeticTarget::B)),
            0x31 => Some(Instruction::Swap(ByteArithmeticTarget::C)),
            0x32 => Some(Instruction::Swap(ByteArithmeticTarget::D)),
            0x33 => Some(Instruction::Swap(ByteArithmeticTarget::E)),
            0x34 => Some(Instruction::Swap(ByteArithmeticTarget::H)),
            0x35 => Some(Instruction::Swap(ByteArithmeticTarget::L)),
            0x36 => Some(Instruction::Swap(ByteArithmeticTarget::HL)),
            0x37 => Some(Instruction::Swap(ByteArithmeticTarget::A)),
            0x38 => Some(Instruction::ShiftRightLogic(ByteArithmeticTarget::B)),
            0x39 => Some(Instruction::ShiftRightLogic(ByteArithmeticTarget::C)),
            0x3a => Some(Instruction::ShiftRightLogic(ByteArithmeticTarget::D)),
            0x3b => Some(Instruction::ShiftRightLogic(ByteArithmeticTarget::E)),
            0x3c => Some(Instruction::ShiftRightLogic(ByteArithmeticTarget::H)),
            0x3d => Some(Instruction::ShiftRightLogic(ByteArithmeticTarget::L)),
            0x3e => Some(Instruction::ShiftRightLogic(ByteArithmeticTarget::HL)),
            0x3f => Some(Instruction::ShiftRightLogic(ByteArithmeticTarget::A)),
            0x40 => Some(Instruction::TestBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::B,
            )),
            0x41 => Some(Instruction::TestBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::C,
            )),
            0x42 => Some(Instruction::TestBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::D,
            )),
            0x43 => Some(Instruction::TestBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::E,
            )),
            0x44 => Some(Instruction::TestBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::H,
            )),
            0x45 => Some(Instruction::TestBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::L,
            )),
            0x46 => Some(Instruction::TestBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::HL,
            )),
            0x47 => Some(Instruction::TestBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::A,
            )),
            0x48 => Some(Instruction::TestBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::B,
            )),
            0x49 => Some(Instruction::TestBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::C,
            )),
            0x4a => Some(Instruction::TestBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::D,
            )),
            0x4b => Some(Instruction::TestBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::E,
            )),
            0x4c => Some(Instruction::TestBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::H,
            )),
            0x4d => Some(Instruction::TestBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::L,
            )),
            0x4e => Some(Instruction::TestBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::HL,
            )),
            0x4f => Some(Instruction::TestBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::A,
            )),
            0x50 => Some(Instruction::TestBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::B,
            )),
            0x51 => Some(Instruction::TestBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::C,
            )),
            0x52 => Some(Instruction::TestBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::D,
            )),
            0x53 => Some(Instruction::TestBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::E,
            )),
            0x54 => Some(Instruction::TestBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::H,
            )),
            0x55 => Some(Instruction::TestBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::L,
            )),
            0x56 => Some(Instruction::TestBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::HL,
            )),
            0x57 => Some(Instruction::TestBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::A,
            )),
            0x58 => Some(Instruction::TestBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::B,
            )),
            0x59 => Some(Instruction::TestBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::C,
            )),
            0x5a => Some(Instruction::TestBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::D,
            )),
            0x5b => Some(Instruction::TestBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::E,
            )),
            0x5c => Some(Instruction::TestBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::H,
            )),
            0x5d => Some(Instruction::TestBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::L,
            )),
            0x5e => Some(Instruction::TestBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::HL,
            )),
            0x5f => Some(Instruction::TestBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::A,
            )),
            0x60 => Some(Instruction::TestBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::B,
            )),
            0x61 => Some(Instruction::TestBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::C,
            )),
            0x62 => Some(Instruction::TestBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::D,
            )),
            0x63 => Some(Instruction::TestBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::E,
            )),
            0x64 => Some(Instruction::TestBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::H,
            )),
            0x65 => Some(Instruction::TestBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::L,
            )),
            0x66 => Some(Instruction::TestBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::HL,
            )),
            0x67 => Some(Instruction::TestBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::A,
            )),
            0x68 => Some(Instruction::TestBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::B,
            )),
            0x69 => Some(Instruction::TestBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::C,
            )),
            0x6a => Some(Instruction::TestBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::D,
            )),
            0x6b => Some(Instruction::TestBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::E,
            )),
            0x6c => Some(Instruction::TestBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::H,
            )),
            0x6d => Some(Instruction::TestBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::L,
            )),
            0x6e => Some(Instruction::TestBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::HL,
            )),
            0x6f => Some(Instruction::TestBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::A,
            )),
            0x70 => Some(Instruction::TestBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::B,
            )),
            0x71 => Some(Instruction::TestBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::C,
            )),
            0x72 => Some(Instruction::TestBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::D,
            )),
            0x73 => Some(Instruction::TestBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::E,
            )),
            0x74 => Some(Instruction::TestBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::H,
            )),
            0x75 => Some(Instruction::TestBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::L,
            )),
            0x76 => Some(Instruction::TestBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::HL,
            )),
            0x77 => Some(Instruction::TestBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::A,
            )),
            0x78 => Some(Instruction::TestBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::B,
            )),
            0x79 => Some(Instruction::TestBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::C,
            )),
            0x7a => Some(Instruction::TestBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::D,
            )),
            0x7b => Some(Instruction::TestBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::E,
            )),
            0x7c => Some(Instruction::TestBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::H,
            )),
            0x7d => Some(Instruction::TestBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::L,
            )),
            0x7e => Some(Instruction::TestBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::HL,
            )),
            0x7f => Some(Instruction::TestBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::A,
            )),
            0x80 => Some(Instruction::ResetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::B,
            )),
            0x81 => Some(Instruction::ResetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::C,
            )),
            0x82 => Some(Instruction::ResetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::D,
            )),
            0x83 => Some(Instruction::ResetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::E,
            )),
            0x84 => Some(Instruction::ResetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::H,
            )),
            0x85 => Some(Instruction::ResetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::L,
            )),
            0x86 => Some(Instruction::ResetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::HL,
            )),
            0x87 => Some(Instruction::ResetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::A,
            )),
            0x88 => Some(Instruction::ResetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::B,
            )),
            0x89 => Some(Instruction::ResetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::C,
            )),
            0x8a => Some(Instruction::ResetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::D,
            )),
            0x8b => Some(Instruction::ResetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::E,
            )),
            0x8c => Some(Instruction::ResetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::H,
            )),
            0x8d => Some(Instruction::ResetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::L,
            )),
            0x8e => Some(Instruction::ResetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::HL,
            )),
            0x8f => Some(Instruction::ResetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::A,
            )),
            0x90 => Some(Instruction::ResetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::B,
            )),
            0x91 => Some(Instruction::ResetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::C,
            )),
            0x92 => Some(Instruction::ResetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::D,
            )),
            0x93 => Some(Instruction::ResetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::E,
            )),
            0x94 => Some(Instruction::ResetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::H,
            )),
            0x95 => Some(Instruction::ResetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::L,
            )),
            0x96 => Some(Instruction::ResetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::HL,
            )),
            0x97 => Some(Instruction::ResetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::A,
            )),
            0x98 => Some(Instruction::ResetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::B,
            )),
            0x99 => Some(Instruction::ResetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::C,
            )),
            0x9a => Some(Instruction::ResetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::D,
            )),
            0x9b => Some(Instruction::ResetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::E,
            )),
            0x9c => Some(Instruction::ResetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::H,
            )),
            0x9d => Some(Instruction::ResetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::L,
            )),
            0x9e => Some(Instruction::ResetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::HL,
            )),
            0x9f => Some(Instruction::ResetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::A,
            )),
            0xa0 => Some(Instruction::ResetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::B,
            )),
            0xa1 => Some(Instruction::ResetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::C,
            )),
            0xa2 => Some(Instruction::ResetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::D,
            )),
            0xa3 => Some(Instruction::ResetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::E,
            )),
            0xa4 => Some(Instruction::ResetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::H,
            )),
            0xa5 => Some(Instruction::ResetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::L,
            )),
            0xa6 => Some(Instruction::ResetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::HL,
            )),
            0xa7 => Some(Instruction::ResetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::A,
            )),
            0xa8 => Some(Instruction::ResetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::B,
            )),
            0xa9 => Some(Instruction::ResetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::C,
            )),
            0xaa => Some(Instruction::ResetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::D,
            )),
            0xab => Some(Instruction::ResetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::E,
            )),
            0xac => Some(Instruction::ResetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::H,
            )),
            0xad => Some(Instruction::ResetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::L,
            )),
            0xae => Some(Instruction::ResetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::HL,
            )),
            0xaf => Some(Instruction::ResetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::A,
            )),
            0xb0 => Some(Instruction::ResetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::B,
            )),
            0xb1 => Some(Instruction::ResetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::C,
            )),
            0xb2 => Some(Instruction::ResetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::D,
            )),
            0xb3 => Some(Instruction::ResetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::E,
            )),
            0xb4 => Some(Instruction::ResetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::H,
            )),
            0xb5 => Some(Instruction::ResetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::L,
            )),
            0xb6 => Some(Instruction::ResetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::HL,
            )),
            0xb7 => Some(Instruction::ResetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::A,
            )),
            0xb8 => Some(Instruction::ResetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::B,
            )),
            0xb9 => Some(Instruction::ResetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::C,
            )),
            0xba => Some(Instruction::ResetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::D,
            )),
            0xbb => Some(Instruction::ResetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::E,
            )),
            0xbc => Some(Instruction::ResetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::H,
            )),
            0xbd => Some(Instruction::ResetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::L,
            )),
            0xbe => Some(Instruction::ResetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::HL,
            )),
            0xbf => Some(Instruction::ResetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::A,
            )),
            0xc0 => Some(Instruction::SetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::B,
            )),
            0xc1 => Some(Instruction::SetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::C,
            )),
            0xc2 => Some(Instruction::SetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::D,
            )),
            0xc3 => Some(Instruction::SetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::E,
            )),
            0xc4 => Some(Instruction::SetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::H,
            )),
            0xc5 => Some(Instruction::SetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::L,
            )),
            0xc6 => Some(Instruction::SetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::HL,
            )),
            0xc7 => Some(Instruction::SetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::A,
            )),
            0xc8 => Some(Instruction::SetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::B,
            )),
            0xc9 => Some(Instruction::SetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::C,
            )),
            0xca => Some(Instruction::SetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::D,
            )),
            0xcb => Some(Instruction::SetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::E,
            )),
            0xcc => Some(Instruction::SetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::H,
            )),
            0xcd => Some(Instruction::SetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::L,
            )),
            0xce => Some(Instruction::SetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::HL,
            )),
            0xcf => Some(Instruction::SetBit(
                BitOpTarget::Bit1,
                ByteArithmeticTarget::A,
            )),
            0xd0 => Some(Instruction::SetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::B,
            )),
            0xd1 => Some(Instruction::SetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::C,
            )),
            0xd2 => Some(Instruction::SetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::D,
            )),
            0xd3 => Some(Instruction::SetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::E,
            )),
            0xd4 => Some(Instruction::SetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::H,
            )),
            0xd5 => Some(Instruction::SetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::L,
            )),
            0xd6 => Some(Instruction::SetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::HL,
            )),
            0xd7 => Some(Instruction::SetBit(
                BitOpTarget::Bit2,
                ByteArithmeticTarget::A,
            )),
            0xd8 => Some(Instruction::SetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::B,
            )),
            0xd9 => Some(Instruction::SetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::C,
            )),
            0xda => Some(Instruction::SetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::D,
            )),
            0xdb => Some(Instruction::SetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::E,
            )),
            0xdc => Some(Instruction::SetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::H,
            )),
            0xdd => Some(Instruction::SetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::L,
            )),
            0xde => Some(Instruction::SetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::HL,
            )),
            0xdf => Some(Instruction::SetBit(
                BitOpTarget::Bit3,
                ByteArithmeticTarget::A,
            )),
            0xe0 => Some(Instruction::SetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::B,
            )),
            0xe1 => Some(Instruction::SetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::C,
            )),
            0xe2 => Some(Instruction::SetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::D,
            )),
            0xe3 => Some(Instruction::SetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::E,
            )),
            0xe4 => Some(Instruction::SetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::H,
            )),
            0xe5 => Some(Instruction::SetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::L,
            )),
            0xe6 => Some(Instruction::SetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::HL,
            )),
            0xe7 => Some(Instruction::SetBit(
                BitOpTarget::Bit4,
                ByteArithmeticTarget::A,
            )),
            0xe8 => Some(Instruction::SetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::B,
            )),
            0xe9 => Some(Instruction::SetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::C,
            )),
            0xea => Some(Instruction::SetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::D,
            )),
            0xeb => Some(Instruction::SetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::E,
            )),
            0xec => Some(Instruction::SetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::H,
            )),
            0xed => Some(Instruction::SetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::L,
            )),
            0xee => Some(Instruction::SetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::HL,
            )),
            0xef => Some(Instruction::SetBit(
                BitOpTarget::Bit5,
                ByteArithmeticTarget::A,
            )),
            0xf0 => Some(Instruction::SetBit(
                BitOpTarget::Bit0,
                ByteArithmeticTarget::B,
            )),
            0xf1 => Some(Instruction::SetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::C,
            )),
            0xf2 => Some(Instruction::SetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::D,
            )),
            0xf3 => Some(Instruction::SetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::E,
            )),
            0xf4 => Some(Instruction::SetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::H,
            )),
            0xf5 => Some(Instruction::SetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::L,
            )),
            0xf6 => Some(Instruction::SetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::HL,
            )),
            0xf7 => Some(Instruction::SetBit(
                BitOpTarget::Bit6,
                ByteArithmeticTarget::A,
            )),
            0xf8 => Some(Instruction::SetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::B,
            )),
            0xf9 => Some(Instruction::SetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::C,
            )),
            0xfa => Some(Instruction::SetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::D,
            )),
            0xfb => Some(Instruction::SetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::E,
            )),
            0xfc => Some(Instruction::SetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::H,
            )),
            0xfd => Some(Instruction::SetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::L,
            )),
            0xfe => Some(Instruction::SetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::HL,
            )),
            0xff => Some(Instruction::SetBit(
                BitOpTarget::Bit7,
                ByteArithmeticTarget::A,
            )),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ByteArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

#[derive(Debug)]
pub enum ArithmeticTarget16 {
    BC,
    DE,
    HL,
    SP,
}
#[derive(Debug)]
pub enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(PartialEq, Debug)]
pub enum LoadTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    ImmediateAddress,
}

#[derive(Debug)]
pub enum MemoryAddressRegistry {
    BC,
    DE,
    HLPlus,
    HLMinus,
}

#[derive(Debug)]
pub enum LoadTarget16 {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug)]
pub enum PushPopTarget {
    BC,
    DE,
    HL,
    AF,
}

#[derive(Debug)]
pub enum BitOpTarget {
    Bit0,
    Bit1,
    Bit2,
    Bit3,
    Bit4,
    Bit5,
    Bit6,
    Bit7,
}
