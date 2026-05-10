//! Decoding helpers for the demo subset.

use crate::{ExternalFlag, Instruction, Reg};
use sw_isa_core::DecodeError;

pub fn decode(bytes: &[u8]) -> Result<(Instruction, usize), DecodeError> {
    let first = *bytes.first().ok_or(DecodeError::Truncated)?;
    match first {
        0x00 => Ok((Instruction::Idle, 1)),
        0x10..=0x1F => Ok((
            Instruction::Increment {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0x30 => {
            let target = *bytes.get(1).ok_or(DecodeError::Truncated)?;
            Ok((Instruction::Branch { target }, 2))
        }
        0x34..=0x37 => {
            let target = *bytes.get(1).ok_or(DecodeError::Truncated)?;
            let flag = ExternalFlag::new(first - 0x33).ok_or(DecodeError::Invalid)?;
            Ok((
                Instruction::BranchExternalFlag {
                    flag,
                    expected: true,
                    target,
                },
                2,
            ))
        }
        0x3C..=0x3F => {
            let target = *bytes.get(1).ok_or(DecodeError::Truncated)?;
            let flag = ExternalFlag::new(first - 0x3B).ok_or(DecodeError::Invalid)?;
            Ok((
                Instruction::BranchExternalFlag {
                    flag,
                    expected: false,
                    target,
                },
                2,
            ))
        }
        0x50..=0x5F => Ok((
            Instruction::Store {
                reg: Reg::new_masked(first),
            },
            1,
        )),
        0x7A => Ok((Instruction::ResetQ, 1)),
        0x7B => Ok((Instruction::SetQ, 1)),
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
        0xF8 => {
            let value = *bytes.get(1).ok_or(DecodeError::Truncated)?;
            Ok((Instruction::LoadImmediate { value }, 2))
        }
        _ => Err(DecodeError::Invalid),
    }
}
