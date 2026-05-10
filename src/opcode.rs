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
    ResetQ = 0x7A,
    SetQ = 0x7B,
    PutLow = 0xA0,
    PutHigh = 0xB0,
    LoadImmediate = 0xF8,
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
            Opcode::ResetQ => "req",
            Opcode::SetQ => "seq",
            Opcode::PutLow => "plo",
            Opcode::PutHigh => "phi",
            Opcode::LoadImmediate => "ldi",
        }
    }

    pub fn size_bytes(self) -> usize {
        match self {
            Opcode::Idle
            | Opcode::Increment
            | Opcode::Store
            | Opcode::ResetQ
            | Opcode::SetQ
            | Opcode::PutLow
            | Opcode::PutHigh => 1,
            Opcode::Branch
            | Opcode::BranchExternalFlag
            | Opcode::BranchNotExternalFlag
            | Opcode::LoadImmediate => 2,
        }
    }
}

impl sw_isa_core::Mnemonic for Opcode {
    fn mnemonic(&self) -> &'static str {
        Opcode::mnemonic(*self)
    }
}
