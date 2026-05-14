//! Encoding helpers for the demo subset.

use crate::{Instruction, Opcode};
use sw_isa_core::EncodeError;

pub fn encode(insn: &Instruction, out: &mut [u8]) -> Result<usize, EncodeError> {
    match *insn {
        Instruction::Idle => encode_one(Opcode::Idle as u8, out),
        Instruction::Increment { reg } => encode_one(Opcode::Increment as u8 | reg.index_u8(), out),
        Instruction::Branch { target } => encode_two(Opcode::Branch as u8, target, out),
        Instruction::BranchExternalFlag {
            flag,
            expected,
            target,
        } => {
            let base = if expected {
                Opcode::BranchExternalFlag as u8
            } else {
                Opcode::BranchNotExternalFlag as u8
            };
            encode_two(base + flag.index_u8() - 1, target, out)
        }
        Instruction::Output { port } if (1..=7).contains(&port) => {
            encode_one(Opcode::Output as u8 | port, out)
        }
        Instruction::Input { port } if (1..=7).contains(&port) => {
            encode_one(Opcode::Input as u8 | port, out)
        }
        Instruction::Output { .. } | Instruction::Input { .. } => Err(EncodeError::InvalidOperands),
        Instruction::Store { reg } => encode_one(Opcode::Store as u8 | reg.index_u8(), out),
        Instruction::ResetQ => encode_one(Opcode::ResetQ as u8, out),
        Instruction::SetQ => encode_one(Opcode::SetQ as u8, out),
        Instruction::GetLow { reg } => encode_one(Opcode::GetLow as u8 | reg.index_u8(), out),
        Instruction::PutLow { reg } => encode_one(Opcode::PutLow as u8 | reg.index_u8(), out),
        Instruction::PutHigh { reg } => encode_one(Opcode::PutHigh as u8 | reg.index_u8(), out),
        Instruction::SetX { reg } => encode_one(Opcode::SetX as u8 | reg.index_u8(), out),
        Instruction::Add => encode_one(Opcode::Add as u8, out),
        Instruction::LoadImmediate { value } => encode_two(Opcode::LoadImmediate as u8, value, out),
        Instruction::AddImmediate { value } => encode_two(Opcode::AddImmediate as u8, value, out),
        Instruction::ShiftLeft => encode_one(Opcode::ShiftLeft as u8, out),
    }
}

fn encode_one(byte: u8, out: &mut [u8]) -> Result<usize, EncodeError> {
    if out.is_empty() {
        return Err(EncodeError::BufferTooSmall);
    }
    out[0] = byte;
    Ok(1)
}

fn encode_two(first: u8, second: u8, out: &mut [u8]) -> Result<usize, EncodeError> {
    if out.len() < 2 {
        return Err(EncodeError::BufferTooSmall);
    }
    out[0] = first;
    out[1] = second;
    Ok(2)
}
