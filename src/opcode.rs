//! Opcode enum and metadata for the demo subset.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Idle = 0x00,
    Increment = 0x10,
    Branch = 0x30,
    BranchExternalFlag = 0x34,
    BranchNotExternalFlag = 0x3C,
    Store = 0x50,
    Output = 0x60,
    Input = 0x68,
    ResetQ = 0x7A,
    SetQ = 0x7B,
    GetLow = 0x80,
    PutLow = 0xA0,
    PutHigh = 0xB0,
    SetX = 0xE0,
    Add = 0xF4,
    LoadImmediate = 0xF8,
    AddImmediate = 0xFC,
    ShiftLeft = 0xFE,
}

impl Opcode {
    pub fn mnemonic(self) -> &'static str {
        match self {
            Opcode::Idle => "idl",
            Opcode::Increment => "inc",
            Opcode::Branch => "br",
            Opcode::BranchExternalFlag => "b-ef",
            Opcode::BranchNotExternalFlag => "bn-ef",
            Opcode::Store => "str",
            Opcode::Output => "out",
            Opcode::Input => "inp",
            Opcode::ResetQ => "req",
            Opcode::SetQ => "seq",
            Opcode::GetLow => "glo",
            Opcode::PutLow => "plo",
            Opcode::PutHigh => "phi",
            Opcode::SetX => "sex",
            Opcode::Add => "add",
            Opcode::LoadImmediate => "ldi",
            Opcode::AddImmediate => "adi",
            Opcode::ShiftLeft => "shl",
        }
    }

    pub fn size_bytes(self) -> usize {
        match self {
            Opcode::Idle
            | Opcode::Increment
            | Opcode::Store
            | Opcode::Output
            | Opcode::Input
            | Opcode::ResetQ
            | Opcode::SetQ
            | Opcode::GetLow
            | Opcode::PutLow
            | Opcode::PutHigh
            | Opcode::SetX
            | Opcode::Add
            | Opcode::ShiftLeft => 1,
            Opcode::Branch
            | Opcode::BranchExternalFlag
            | Opcode::BranchNotExternalFlag
            | Opcode::LoadImmediate
            | Opcode::AddImmediate => 2,
        }
    }
}

impl sw_isa_core::Mnemonic for Opcode {
    fn mnemonic(&self) -> &'static str {
        Opcode::mnemonic(*self)
    }
}
