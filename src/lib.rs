//! `sw-cdp1802-isa`: RCA CDP1802 ISA description: opcodes, encoding, decoding, disassembly.
//!
//! Full CDP1802 ISA implementation against the trait surfaces in `sw-langtools`.

pub mod branch;
pub mod decode;
pub mod encode;
pub mod opcode;
pub mod register;

pub use branch::{ExternalFlag, LongBranchCondition, LongSkipCondition};
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
    LoadVia {
        reg: Reg,
    },
    Increment {
        reg: Reg,
    },
    Decrement {
        reg: Reg,
    },
    Branch {
        target: u8,
    },
    BranchQ {
        expected: bool,
        target: u8,
    },
    BranchZero {
        expected: bool,
        target: u8,
    },
    BranchDataFlag {
        expected: bool,
        target: u8,
    },
    BranchExternalFlag {
        flag: ExternalFlag,
        expected: bool,
        target: u8,
    },
    ShortSkip {
        filler: u8,
    },
    LoadAdvance {
        reg: Reg,
    },
    Output {
        port: u8,
    },
    Input {
        port: u8,
    },
    Irx,
    Reserved68,
    Store {
        reg: Reg,
    },
    Return,
    DisableInterrupt,
    LoadViaXAdvance,
    StoreViaXDecrement,
    AddWithCarry,
    SubtractDWithBorrow,
    ShiftRightWithCarry,
    SubtractMemoryWithBorrow,
    Save,
    Mark,
    ResetQ,
    SetQ,
    AddWithCarryImmediate {
        value: u8,
    },
    SubtractDWithBorrowImmediate {
        value: u8,
    },
    ShiftLeftWithCarry,
    SubtractMemoryWithBorrowImmediate {
        value: u8,
    },
    GetLow {
        reg: Reg,
    },
    GetHigh {
        reg: Reg,
    },
    PutLow {
        reg: Reg,
    },
    PutHigh {
        reg: Reg,
    },
    LoadImmediate {
        value: u8,
    },
    SetX {
        reg: Reg,
    },
    LongBranch {
        condition: LongBranchCondition,
        target: u16,
    },
    NoOperation,
    LongSkip {
        condition: LongSkipCondition,
    },
    SetP {
        reg: Reg,
    },
    LoadViaX,
    Or,
    And,
    Xor,
    Add,
    SubtractDNoBorrow,
    ShiftRight,
    SubtractMemoryNoBorrow,
    AddImmediate {
        value: u8,
    },
    OrImmediate {
        value: u8,
    },
    AndImmediate {
        value: u8,
    },
    XorImmediate {
        value: u8,
    },
    SubtractDImmediateNoBorrow {
        value: u8,
    },
    ShiftLeft,
    SubtractMemoryNoBorrowImmediate {
        value: u8,
    },
}

impl Instruction {
    pub fn opcode(self) -> Opcode {
        match self {
            Instruction::Idle => Opcode::Idle,
            Instruction::LoadVia { .. } => Opcode::LoadVia,
            Instruction::Increment { .. } => Opcode::Increment,
            Instruction::Decrement { .. } => Opcode::Decrement,
            Instruction::Branch { .. } => Opcode::Branch,
            Instruction::BranchQ { expected, .. } => {
                if expected {
                    Opcode::BranchQ
                } else {
                    Opcode::BranchNotQ
                }
            }
            Instruction::BranchZero { expected, .. } => {
                if expected {
                    Opcode::BranchZero
                } else {
                    Opcode::BranchNotZero
                }
            }
            Instruction::BranchDataFlag { expected, .. } => {
                if expected {
                    Opcode::BranchDataFlag
                } else {
                    Opcode::BranchNotDataFlag
                }
            }
            Instruction::BranchExternalFlag { expected, .. } => {
                if expected {
                    Opcode::BranchExternalFlag
                } else {
                    Opcode::BranchNotExternalFlag
                }
            }
            Instruction::ShortSkip { .. } => Opcode::ShortSkip,
            Instruction::LoadAdvance { .. } => Opcode::LoadAdvance,
            Instruction::Output { .. } => Opcode::Output,
            Instruction::Input { .. } => Opcode::Input,
            Instruction::Irx => Opcode::Irx,
            Instruction::Reserved68 => Opcode::Reserved68,
            Instruction::Store { .. } => Opcode::Store,
            Instruction::Return => Opcode::Return,
            Instruction::DisableInterrupt => Opcode::DisableInterrupt,
            Instruction::LoadViaXAdvance => Opcode::LoadViaXAdvance,
            Instruction::StoreViaXDecrement => Opcode::StoreViaXDecrement,
            Instruction::AddWithCarry => Opcode::AddWithCarry,
            Instruction::SubtractDWithBorrow => Opcode::SubtractDWithBorrow,
            Instruction::ShiftRightWithCarry => Opcode::ShiftRightWithCarry,
            Instruction::SubtractMemoryWithBorrow => Opcode::SubtractMemoryWithBorrow,
            Instruction::Save => Opcode::Save,
            Instruction::Mark => Opcode::Mark,
            Instruction::ResetQ => Opcode::ResetQ,
            Instruction::SetQ => Opcode::SetQ,
            Instruction::AddWithCarryImmediate { .. } => Opcode::AddWithCarryImmediate,
            Instruction::SubtractDWithBorrowImmediate { .. } => {
                Opcode::SubtractDWithBorrowImmediate
            }
            Instruction::ShiftLeftWithCarry => Opcode::ShiftLeftWithCarry,
            Instruction::SubtractMemoryWithBorrowImmediate { .. } => {
                Opcode::SubtractMemoryWithBorrowImmediate
            }
            Instruction::GetLow { .. } => Opcode::GetLow,
            Instruction::GetHigh { .. } => Opcode::GetHigh,
            Instruction::PutLow { .. } => Opcode::PutLow,
            Instruction::PutHigh { .. } => Opcode::PutHigh,
            Instruction::LoadImmediate { .. } => Opcode::LoadImmediate,
            Instruction::SetX { .. } => Opcode::SetX,
            Instruction::LongBranch { condition, .. } => match condition {
                LongBranchCondition::Always => Opcode::LongBranch,
                LongBranchCondition::Q => Opcode::LongBranchQ,
                LongBranchCondition::Zero => Opcode::LongBranchZero,
                LongBranchCondition::DataFlag => Opcode::LongBranchDataFlag,
                LongBranchCondition::NotQ => Opcode::LongBranchNotQ,
                LongBranchCondition::NotZero => Opcode::LongBranchNotZero,
                LongBranchCondition::NotDataFlag => Opcode::LongBranchNotDataFlag,
            },
            Instruction::NoOperation => Opcode::NoOperation,
            Instruction::LongSkip { condition } => match condition {
                LongSkipCondition::Always => Opcode::LongSkip,
                LongSkipCondition::Q => Opcode::LongSkipQ,
                LongSkipCondition::Zero => Opcode::LongSkipZero,
                LongSkipCondition::DataFlag => Opcode::LongSkipDataFlag,
                LongSkipCondition::NotQ => Opcode::LongSkipNotQ,
                LongSkipCondition::NotZero => Opcode::LongSkipNotZero,
                LongSkipCondition::NotDataFlag => Opcode::LongSkipNotDataFlag,
                LongSkipCondition::InterruptEnabled => Opcode::LongSkipInterruptEnabled,
            },
            Instruction::SetP { .. } => Opcode::SetP,
            Instruction::LoadViaX => Opcode::LoadViaX,
            Instruction::Or => Opcode::Or,
            Instruction::And => Opcode::And,
            Instruction::Xor => Opcode::Xor,
            Instruction::Add => Opcode::Add,
            Instruction::SubtractDNoBorrow => Opcode::SubtractDNoBorrow,
            Instruction::ShiftRight => Opcode::ShiftRight,
            Instruction::SubtractMemoryNoBorrow => Opcode::SubtractMemoryNoBorrow,
            Instruction::AddImmediate { .. } => Opcode::AddImmediate,
            Instruction::OrImmediate { .. } => Opcode::OrImmediate,
            Instruction::AndImmediate { .. } => Opcode::AndImmediate,
            Instruction::XorImmediate { .. } => Opcode::XorImmediate,
            Instruction::SubtractDImmediateNoBorrow { .. } => Opcode::SubtractDImmediateNoBorrow,
            Instruction::SubtractMemoryNoBorrowImmediate { .. } => {
                Opcode::SubtractMemoryNoBorrowImmediate
            }
            Instruction::ShiftLeft => Opcode::ShiftLeft,
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
    const MAX_INSTR_BYTES: usize = 3;

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
            Instruction::LoadVia { reg } => write!(w, "ldn {}", reg.name()),
            Instruction::Increment { reg } => write!(w, "inc {}", reg.name()),
            Instruction::Decrement { reg } => write!(w, "dec {}", reg.name()),
            Instruction::Branch { target } => write!(w, "br 0x{target:02x}"),
            Instruction::BranchQ { expected, target } => {
                if *expected {
                    write!(w, "bq 0x{target:02x}")
                } else {
                    write!(w, "bnq 0x{target:02x}")
                }
            }
            Instruction::BranchZero { expected, target } => {
                if *expected {
                    write!(w, "bz 0x{target:02x}")
                } else {
                    write!(w, "bnz 0x{target:02x}")
                }
            }
            Instruction::BranchDataFlag { expected, target } => {
                if *expected {
                    write!(w, "bdf 0x{target:02x}")
                } else {
                    write!(w, "bnf 0x{target:02x}")
                }
            }
            Instruction::BranchExternalFlag {
                flag,
                expected,
                target,
            } => {
                let prefix = if *expected { "b" } else { "bn" };
                write!(w, "{prefix}{} 0x{target:02x}", flag.index_u8())
            }
            Instruction::ShortSkip { filler } => write!(w, "skp 0x{filler:02x}"),
            Instruction::LoadAdvance { reg } => write!(w, "lda {}", reg.name()),
            Instruction::Output { port } => write!(w, "out {port}"),
            Instruction::Input { port } => write!(w, "inp {port}"),
            Instruction::Irx => write!(w, "irx"),
            Instruction::Reserved68 => write!(w, "reserved68"),
            Instruction::Store { reg } => write!(w, "str {}", reg.name()),
            Instruction::Return => write!(w, "ret"),
            Instruction::DisableInterrupt => write!(w, "dis"),
            Instruction::LoadViaXAdvance => write!(w, "ldxa"),
            Instruction::StoreViaXDecrement => write!(w, "stxd"),
            Instruction::AddWithCarry => write!(w, "adc"),
            Instruction::SubtractDWithBorrow => write!(w, "sdb"),
            Instruction::ShiftRightWithCarry => write!(w, "shrc"),
            Instruction::SubtractMemoryWithBorrow => write!(w, "smb"),
            Instruction::Save => write!(w, "sav"),
            Instruction::Mark => write!(w, "mark"),
            Instruction::ResetQ => write!(w, "req"),
            Instruction::SetQ => write!(w, "seq"),
            Instruction::AddWithCarryImmediate { value } => write!(w, "adci 0x{value:02x}"),
            Instruction::SubtractDWithBorrowImmediate { value } => {
                write!(w, "sdbi 0x{value:02x}")
            }
            Instruction::ShiftLeftWithCarry => write!(w, "shlc"),
            Instruction::SubtractMemoryWithBorrowImmediate { value } => {
                write!(w, "smbi 0x{value:02x}")
            }
            Instruction::GetLow { reg } => write!(w, "glo {}", reg.name()),
            Instruction::GetHigh { reg } => write!(w, "ghi {}", reg.name()),
            Instruction::PutLow { reg } => write!(w, "plo {}", reg.name()),
            Instruction::PutHigh { reg } => write!(w, "phi {}", reg.name()),
            Instruction::LoadImmediate { value } => write!(w, "ldi 0x{value:02x}"),
            Instruction::SetX { reg } => write!(w, "sex {}", reg.name()),
            Instruction::LongBranch { condition, target } => {
                write!(w, "{} 0x{target:04x}", long_branch_mnemonic(*condition))
            }
            Instruction::NoOperation => write!(w, "nop"),
            Instruction::LongSkip { condition } => write!(w, "{}", long_skip_mnemonic(*condition)),
            Instruction::SetP { reg } => write!(w, "sep {}", reg.name()),
            Instruction::LoadViaX => write!(w, "ldx"),
            Instruction::Or => write!(w, "or"),
            Instruction::And => write!(w, "and"),
            Instruction::Xor => write!(w, "xor"),
            Instruction::Add => write!(w, "add"),
            Instruction::SubtractDNoBorrow => write!(w, "sd"),
            Instruction::ShiftRight => write!(w, "shr"),
            Instruction::SubtractMemoryNoBorrow => write!(w, "sm"),
            Instruction::AddImmediate { value } => write!(w, "adi 0x{value:02x}"),
            Instruction::OrImmediate { value } => write!(w, "ori 0x{value:02x}"),
            Instruction::AndImmediate { value } => write!(w, "ani 0x{value:02x}"),
            Instruction::XorImmediate { value } => write!(w, "xri 0x{value:02x}"),
            Instruction::SubtractDImmediateNoBorrow { value } => {
                write!(w, "sdi 0x{value:02x}")
            }
            Instruction::SubtractMemoryNoBorrowImmediate { value } => {
                write!(w, "smi 0x{value:02x}")
            }
            Instruction::ShiftLeft => write!(w, "shl"),
        }
    }
}

fn long_branch_mnemonic(condition: LongBranchCondition) -> &'static str {
    match condition {
        LongBranchCondition::Always => "lbr",
        LongBranchCondition::Q => "lbq",
        LongBranchCondition::Zero => "lbz",
        LongBranchCondition::DataFlag => "lbdf",
        LongBranchCondition::NotQ => "lbnq",
        LongBranchCondition::NotZero => "lbnz",
        LongBranchCondition::NotDataFlag => "lbnf",
    }
}

fn long_skip_mnemonic(condition: LongSkipCondition) -> &'static str {
    match condition {
        LongSkipCondition::Always => "lskp",
        LongSkipCondition::Q => "lsq",
        LongSkipCondition::Zero => "lsz",
        LongSkipCondition::DataFlag => "lsdf",
        LongSkipCondition::NotQ => "lsnq",
        LongSkipCondition::NotZero => "lsnz",
        LongSkipCondition::NotDataFlag => "lsnf",
        LongSkipCondition::InterruptEnabled => "lsie",
    }
}
