//! Opcode enum and metadata for the full CDP1802 instruction set.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Idle = 0x00,
    LoadVia = 0x01,
    Increment = 0x10,
    Decrement = 0x20,
    Branch = 0x30,
    BranchQ = 0x31,
    BranchZero = 0x32,
    BranchDataFlag = 0x33,
    BranchExternalFlag = 0x34,
    ShortSkip = 0x38,
    BranchNotQ = 0x39,
    BranchNotZero = 0x3A,
    BranchNotDataFlag = 0x3B,
    BranchNotExternalFlag = 0x3C,
    LoadAdvance = 0x40,
    Store = 0x50,
    Irx = 0x60,
    Output = 0x61,
    Reserved68 = 0x68,
    Input = 0x69,
    Return = 0x70,
    DisableInterrupt = 0x71,
    LoadViaXAdvance = 0x72,
    StoreViaXDecrement = 0x73,
    AddWithCarry = 0x74,
    SubtractDWithBorrow = 0x75,
    ShiftRightWithCarry = 0x76,
    SubtractMemoryWithBorrow = 0x77,
    Save = 0x78,
    Mark = 0x79,
    ResetQ = 0x7A,
    SetQ = 0x7B,
    AddWithCarryImmediate = 0x7C,
    SubtractDWithBorrowImmediate = 0x7D,
    ShiftLeftWithCarry = 0x7E,
    SubtractMemoryWithBorrowImmediate = 0x7F,
    GetLow = 0x80,
    GetHigh = 0x90,
    PutLow = 0xA0,
    PutHigh = 0xB0,
    LongBranch = 0xC0,
    LongBranchQ = 0xC1,
    LongBranchZero = 0xC2,
    LongBranchDataFlag = 0xC3,
    NoOperation = 0xC4,
    LongSkipNotQ = 0xC5,
    LongSkipNotZero = 0xC6,
    LongSkipNotDataFlag = 0xC7,
    LongSkip = 0xC8,
    LongBranchNotQ = 0xC9,
    LongBranchNotZero = 0xCA,
    LongBranchNotDataFlag = 0xCB,
    LongSkipInterruptEnabled = 0xCC,
    LongSkipQ = 0xCD,
    LongSkipZero = 0xCE,
    LongSkipDataFlag = 0xCF,
    SetP = 0xD0,
    SetX = 0xE0,
    LoadViaX = 0xF0,
    Or = 0xF1,
    And = 0xF2,
    Xor = 0xF3,
    Add = 0xF4,
    SubtractDNoBorrow = 0xF5,
    ShiftRight = 0xF6,
    SubtractMemoryNoBorrow = 0xF7,
    LoadImmediate = 0xF8,
    OrImmediate = 0xF9,
    AndImmediate = 0xFA,
    XorImmediate = 0xFB,
    AddImmediate = 0xFC,
    SubtractDImmediateNoBorrow = 0xFD,
    ShiftLeft = 0xFE,
    SubtractMemoryNoBorrowImmediate = 0xFF,
}

impl Opcode {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Opcode::Idle => "idl",
            Opcode::LoadVia => "ldn",
            Opcode::Increment => "inc",
            Opcode::Decrement => "dec",
            Opcode::Branch => "br",
            Opcode::BranchQ => "bq",
            Opcode::BranchZero => "bz",
            Opcode::BranchDataFlag => "bdf",
            Opcode::BranchExternalFlag => "b-ef",
            Opcode::ShortSkip => "skp",
            Opcode::BranchNotQ => "bnq",
            Opcode::BranchNotZero => "bnz",
            Opcode::BranchNotDataFlag => "bnf",
            Opcode::BranchNotExternalFlag => "bn-ef",
            Opcode::LoadAdvance => "lda",
            Opcode::Store => "str",
            Opcode::Irx => "irx",
            Opcode::Output => "out",
            Opcode::Reserved68 => "reserved68",
            Opcode::Input => "inp",
            Opcode::Return => "ret",
            Opcode::DisableInterrupt => "dis",
            Opcode::LoadViaXAdvance => "ldxa",
            Opcode::StoreViaXDecrement => "stxd",
            Opcode::AddWithCarry => "adc",
            Opcode::SubtractDWithBorrow => "sdb",
            Opcode::ShiftRightWithCarry => "shrc",
            Opcode::SubtractMemoryWithBorrow => "smb",
            Opcode::Save => "sav",
            Opcode::Mark => "mark",
            Opcode::ResetQ => "req",
            Opcode::SetQ => "seq",
            Opcode::AddWithCarryImmediate => "adci",
            Opcode::SubtractDWithBorrowImmediate => "sdbi",
            Opcode::ShiftLeftWithCarry => "shlc",
            Opcode::SubtractMemoryWithBorrowImmediate => "smbi",
            Opcode::GetLow => "glo",
            Opcode::GetHigh => "ghi",
            Opcode::PutLow => "plo",
            Opcode::PutHigh => "phi",
            Opcode::LongBranch => "lbr",
            Opcode::LongBranchQ => "lbq",
            Opcode::LongBranchZero => "lbz",
            Opcode::LongBranchDataFlag => "lbdf",
            Opcode::NoOperation => "nop",
            Opcode::LongSkipNotQ => "lsnq",
            Opcode::LongSkipNotZero => "lsnz",
            Opcode::LongSkipNotDataFlag => "lsnf",
            Opcode::LongSkip => "lskp",
            Opcode::LongBranchNotQ => "lbnq",
            Opcode::LongBranchNotZero => "lbnz",
            Opcode::LongBranchNotDataFlag => "lbnf",
            Opcode::LongSkipInterruptEnabled => "lsie",
            Opcode::LongSkipQ => "lsq",
            Opcode::LongSkipZero => "lsz",
            Opcode::LongSkipDataFlag => "lsdf",
            Opcode::SetP => "sep",
            Opcode::SetX => "sex",
            Opcode::LoadViaX => "ldx",
            Opcode::Or => "or",
            Opcode::And => "and",
            Opcode::Xor => "xor",
            Opcode::Add => "add",
            Opcode::SubtractDNoBorrow => "sd",
            Opcode::ShiftRight => "shr",
            Opcode::SubtractMemoryNoBorrow => "sm",
            Opcode::LoadImmediate => "ldi",
            Opcode::OrImmediate => "ori",
            Opcode::AndImmediate => "ani",
            Opcode::XorImmediate => "xri",
            Opcode::AddImmediate => "adi",
            Opcode::SubtractDImmediateNoBorrow => "sdi",
            Opcode::ShiftLeft => "shl",
            Opcode::SubtractMemoryNoBorrowImmediate => "smi",
        }
    }

    pub fn size_bytes(self) -> usize {
        match self {
            Opcode::Branch
            | Opcode::BranchQ
            | Opcode::BranchZero
            | Opcode::BranchDataFlag
            | Opcode::BranchExternalFlag
            | Opcode::ShortSkip
            | Opcode::BranchNotQ
            | Opcode::BranchNotZero
            | Opcode::BranchNotDataFlag
            | Opcode::BranchNotExternalFlag
            | Opcode::AddWithCarryImmediate
            | Opcode::SubtractDWithBorrowImmediate
            | Opcode::SubtractMemoryWithBorrowImmediate
            | Opcode::LoadImmediate
            | Opcode::OrImmediate
            | Opcode::AndImmediate
            | Opcode::XorImmediate
            | Opcode::AddImmediate
            | Opcode::SubtractDImmediateNoBorrow
            | Opcode::SubtractMemoryNoBorrowImmediate => 2,
            Opcode::LongBranch
            | Opcode::LongBranchQ
            | Opcode::LongBranchZero
            | Opcode::LongBranchDataFlag
            | Opcode::LongBranchNotQ
            | Opcode::LongBranchNotZero
            | Opcode::LongBranchNotDataFlag => 3,
            _ => 1,
        }
    }
}

impl sw_isa_core::Mnemonic for Opcode {
    fn mnemonic(&self) -> &'static str {
        Opcode::mnemonic(*self)
    }
}
