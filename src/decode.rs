//! Decoding helpers for the full CDP1802 ISA.

use crate::{ExternalFlag, Instruction, LongBranchCondition, LongSkipCondition, Reg};
use sw_isa_core::DecodeError;

pub fn decode(bytes: &[u8]) -> Result<(Instruction, usize), DecodeError> {
    let first = *bytes.first().ok_or(DecodeError::Truncated)?;
    match first {
        0x00 => Ok((Instruction::Idle, 1)),
        0x01..=0x0F => Ok((
            Instruction::LoadVia {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0x10..=0x1F => Ok((
            Instruction::Increment {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0x20..=0x2F => Ok((
            Instruction::Decrement {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0x30 => decode_short(bytes, |target| Instruction::Branch { target }),
        0x31 => decode_short(bytes, |target| Instruction::BranchQ {
            expected: true,
            target,
        }),
        0x32 => decode_short(bytes, |target| Instruction::BranchZero {
            expected: true,
            target,
        }),
        0x33 => decode_short(bytes, |target| Instruction::BranchDataFlag {
            expected: true,
            target,
        }),
        0x34..=0x37 => decode_short(bytes, |target| Instruction::BranchExternalFlag {
            flag: ExternalFlag::new(first - 0x33).expect("EF opcode maps to EF1..EF4"),
            expected: true,
            target,
        }),
        0x38 => decode_short(bytes, |filler| Instruction::ShortSkip { filler }),
        0x39 => decode_short(bytes, |target| Instruction::BranchQ {
            expected: false,
            target,
        }),
        0x3A => decode_short(bytes, |target| Instruction::BranchZero {
            expected: false,
            target,
        }),
        0x3B => decode_short(bytes, |target| Instruction::BranchDataFlag {
            expected: false,
            target,
        }),
        0x3C..=0x3F => decode_short(bytes, |target| Instruction::BranchExternalFlag {
            flag: ExternalFlag::new(first - 0x3B).expect("EF opcode maps to EF1..EF4"),
            expected: false,
            target,
        }),
        0x40..=0x4F => Ok((
            Instruction::LoadAdvance {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0x50..=0x5F => Ok((
            Instruction::Store {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0x60 => Ok((Instruction::Irx, 1)),
        0x61..=0x67 => Ok((Instruction::Output { port: first & 0x07 }, 1)),
        0x68 => Ok((Instruction::Reserved68, 1)),
        0x69..=0x6F => Ok((Instruction::Input { port: first & 0x07 }, 1)),
        0x70 => Ok((Instruction::Return, 1)),
        0x71 => Ok((Instruction::DisableInterrupt, 1)),
        0x72 => Ok((Instruction::LoadViaXAdvance, 1)),
        0x73 => Ok((Instruction::StoreViaXDecrement, 1)),
        0x74 => Ok((Instruction::AddWithCarry, 1)),
        0x75 => Ok((Instruction::SubtractDWithBorrow, 1)),
        0x76 => Ok((Instruction::ShiftRightWithCarry, 1)),
        0x77 => Ok((Instruction::SubtractMemoryWithBorrow, 1)),
        0x78 => Ok((Instruction::Save, 1)),
        0x79 => Ok((Instruction::Mark, 1)),
        0x7A => Ok((Instruction::ResetQ, 1)),
        0x7B => Ok((Instruction::SetQ, 1)),
        0x7C => decode_immediate(bytes, |value| Instruction::AddWithCarryImmediate { value }),
        0x7D => decode_immediate(bytes, |value| Instruction::SubtractDWithBorrowImmediate {
            value,
        }),
        0x7E => Ok((Instruction::ShiftLeftWithCarry, 1)),
        0x7F => decode_immediate(bytes, |value| {
            Instruction::SubtractMemoryWithBorrowImmediate { value }
        }),
        0x80..=0x8F => Ok((
            Instruction::GetLow {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0x90..=0x9F => Ok((
            Instruction::GetHigh {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0xA0..=0xAF => Ok((
            Instruction::PutLow {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0xB0..=0xBF => Ok((
            Instruction::PutHigh {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0xC0 => decode_long(bytes, LongBranchCondition::Always),
        0xC1 => decode_long(bytes, LongBranchCondition::Q),
        0xC2 => decode_long(bytes, LongBranchCondition::Zero),
        0xC3 => decode_long(bytes, LongBranchCondition::DataFlag),
        0xC4 => Ok((Instruction::NoOperation, 1)),
        0xC5 => Ok((
            Instruction::LongSkip {
                condition: LongSkipCondition::NotQ,
            },
            1,
        )),
        0xC6 => Ok((
            Instruction::LongSkip {
                condition: LongSkipCondition::NotZero,
            },
            1,
        )),
        0xC7 => Ok((
            Instruction::LongSkip {
                condition: LongSkipCondition::NotDataFlag,
            },
            1,
        )),
        0xC8 => Ok((
            Instruction::LongSkip {
                condition: LongSkipCondition::Always,
            },
            1,
        )),
        0xC9 => decode_long(bytes, LongBranchCondition::NotQ),
        0xCA => decode_long(bytes, LongBranchCondition::NotZero),
        0xCB => decode_long(bytes, LongBranchCondition::NotDataFlag),
        0xCC => Ok((
            Instruction::LongSkip {
                condition: LongSkipCondition::InterruptEnabled,
            },
            1,
        )),
        0xCD => Ok((
            Instruction::LongSkip {
                condition: LongSkipCondition::Q,
            },
            1,
        )),
        0xCE => Ok((
            Instruction::LongSkip {
                condition: LongSkipCondition::Zero,
            },
            1,
        )),
        0xCF => Ok((
            Instruction::LongSkip {
                condition: LongSkipCondition::DataFlag,
            },
            1,
        )),
        0xD0..=0xDF => Ok((
            Instruction::SetP {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0xE0..=0xEF => Ok((
            Instruction::SetX {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0xF0 => Ok((Instruction::LoadViaX, 1)),
        0xF1 => Ok((Instruction::Or, 1)),
        0xF2 => Ok((Instruction::And, 1)),
        0xF3 => Ok((Instruction::Xor, 1)),
        0xF4 => Ok((Instruction::Add, 1)),
        0xF5 => Ok((Instruction::SubtractDNoBorrow, 1)),
        0xF6 => Ok((Instruction::ShiftRight, 1)),
        0xF7 => Ok((Instruction::SubtractMemoryNoBorrow, 1)),
        0xF8 => decode_immediate(bytes, |value| Instruction::LoadImmediate { value }),
        0xF9 => decode_immediate(bytes, |value| Instruction::OrImmediate { value }),
        0xFA => decode_immediate(bytes, |value| Instruction::AndImmediate { value }),
        0xFB => decode_immediate(bytes, |value| Instruction::XorImmediate { value }),
        0xFC => decode_immediate(bytes, |value| Instruction::AddImmediate { value }),
        0xFD => decode_immediate(bytes, |value| Instruction::SubtractDImmediateNoBorrow {
            value,
        }),
        0xFE => Ok((Instruction::ShiftLeft, 1)),
        0xFF => decode_immediate(bytes, |value| {
            Instruction::SubtractMemoryNoBorrowImmediate { value }
        }),
    }
}

fn decode_short(
    bytes: &[u8],
    build: impl FnOnce(u8) -> Instruction,
) -> Result<(Instruction, usize), DecodeError> {
    let target = *bytes.get(1).ok_or(DecodeError::Truncated)?;
    Ok((build(target), 2))
}

fn decode_immediate(
    bytes: &[u8],
    build: impl FnOnce(u8) -> Instruction,
) -> Result<(Instruction, usize), DecodeError> {
    let value = *bytes.get(1).ok_or(DecodeError::Truncated)?;
    Ok((build(value), 2))
}

fn decode_long(
    bytes: &[u8],
    condition: LongBranchCondition,
) -> Result<(Instruction, usize), DecodeError> {
    let high = *bytes.get(1).ok_or(DecodeError::Truncated)?;
    let low = *bytes.get(2).ok_or(DecodeError::Truncated)?;
    Ok((
        Instruction::LongBranch {
            condition,
            target: u16::from_be_bytes([high, low]),
        },
        3,
    ))
}
