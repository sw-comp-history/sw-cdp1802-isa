//! `sw-cdp1802-isa`: RCA CDP1802 ISA description: opcodes, encoding, decoding, disassembly.
//!
//! Demo-subset ISA implementation against the trait surfaces in
//! `sw-langtools`.

pub mod branch;
pub mod decode;
pub mod encode;
pub mod opcode;
pub mod register;

pub use branch::ExternalFlag;
pub use opcode::Opcode;
pub use register::{Reg, parse_register};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr(pub u32);

impl sw_isa_core::address::AddressType for Addr {
    fn to_u64(self) -> u64 {
        self.0 as u64
    }

    fn from_u64(v: u64) -> Self {
        Addr(v as u32)
    }

    fn step(self, n: i64) -> Self {
        Addr((self.0 as i64 + n) as u32)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cdp1802;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Idle,
    Increment {
        reg: Reg,
    },
    Branch {
        target: u8,
    },
    BranchExternalFlag {
        flag: ExternalFlag,
        expected: bool,
        target: u8,
    },
    Output {
        port: u8,
    },
    Input {
        port: u8,
    },
    Store {
        reg: Reg,
    },
    ResetQ,
    SetQ,
    PutLow {
        reg: Reg,
    },
    PutHigh {
        reg: Reg,
    },
    LoadImmediate {
        value: u8,
    },
}

impl Instruction {
    pub fn opcode(self) -> Opcode {
        match self {
            Instruction::Idle => Opcode::Idle,
            Instruction::Increment { .. } => Opcode::Increment,
            Instruction::Branch { .. } => Opcode::Branch,
            Instruction::BranchExternalFlag { expected, .. } => {
                if expected {
                    Opcode::BranchExternalFlag
                } else {
                    Opcode::BranchNotExternalFlag
                }
            }
            Instruction::Output { .. } => Opcode::Output,
            Instruction::Input { .. } => Opcode::Input,
            Instruction::Store { .. } => Opcode::Store,
            Instruction::ResetQ => Opcode::ResetQ,
            Instruction::SetQ => Opcode::SetQ,
            Instruction::PutLow { .. } => Opcode::PutLow,
            Instruction::PutHigh { .. } => Opcode::PutHigh,
            Instruction::LoadImmediate { .. } => Opcode::LoadImmediate,
        }
    }
}

impl sw_isa_core::Architecture for Cdp1802 {
    type Opcode = opcode::Opcode;
    type Register = register::Reg;
    type Instruction = Instruction;
    type Address = Addr;
    type Format = sw_isa_core::format::Length;

    const NAME: &'static str = "RCA CDP1802";
    const ENDIAN: sw_isa_core::endian::Endian = sw_isa_core::endian::Endian::Big;
    const ADDRESS_UNIT: sw_isa_core::address::AddressUnit = sw_isa_core::address::AddressUnit::Byte;
    const WORD_BITS: u32 = 8;
    const MIN_INSTR_BYTES: usize = 1;
    const MAX_INSTR_BYTES: usize = 2;

    fn decode(
        bytes: &[u8],
        _pc: Self::Address,
    ) -> Result<(Self::Instruction, usize), sw_isa_core::DecodeError> {
        decode::decode(bytes)
    }

    fn encode(insn: &Self::Instruction, out: &mut [u8]) -> Result<usize, sw_isa_core::EncodeError> {
        encode::encode(insn, out)
    }

    fn disassemble(insn: &Self::Instruction, w: &mut dyn core::fmt::Write) -> core::fmt::Result {
        match insn {
            Instruction::Idle => write!(w, "idl"),
            Instruction::Increment { reg } => write!(w, "inc {}", reg.name()),
            Instruction::Branch { target } => write!(w, "br 0x{target:02x}"),
            Instruction::BranchExternalFlag {
                flag,
                expected,
                target,
            } => {
                let prefix = if *expected { "b" } else { "bn" };
                write!(w, "{prefix}{} 0x{target:02x}", flag.index_u8())
            }
            Instruction::Output { port } => write!(w, "out {port}"),
            Instruction::Input { port } => write!(w, "inp {port}"),
            Instruction::Store { reg } => write!(w, "str {}", reg.name()),
            Instruction::ResetQ => write!(w, "req"),
            Instruction::SetQ => write!(w, "seq"),
            Instruction::PutLow { reg } => write!(w, "plo {}", reg.name()),
            Instruction::PutHigh { reg } => write!(w, "phi {}", reg.name()),
            Instruction::LoadImmediate { value } => write!(w, "ldi 0x{value:02x}"),
        }
    }
}
