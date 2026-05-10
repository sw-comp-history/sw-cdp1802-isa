//! Opcode enum and metadata for the demo subset.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Idle = 0x00,
    Increment = 0x10,
    Branch = 0x30,
    Store = 0x50,
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
            Opcode::Store => "str",
            Opcode::PutLow => "plo",
            Opcode::PutHigh => "phi",
            Opcode::LoadImmediate => "ldi",
        }
    }

    pub fn size_bytes(self) -> usize {
        match self {
            Opcode::Idle | Opcode::Increment | Opcode::Store | Opcode::PutLow | Opcode::PutHigh => {
                1
            }
            Opcode::Branch | Opcode::LoadImmediate => 2,
        }
    }
}

impl sw_isa_core::Mnemonic for Opcode {
    fn mnemonic(&self) -> &'static str {
        Opcode::mnemonic(*self)
    }
}
