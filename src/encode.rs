//! Encoding helpers for the full CDP1802 ISA.

use crate::{Instruction, LongBranchCondition, LongSkipCondition, Opcode};
use sw_isa_core::EncodeError;

pub fn encode(insn: &Instruction, out: &mut [u8]) -> Result<usize, EncodeError> {
    match *insn {
        Instruction::Idle => encode_one(Opcode::Idle as u8, out),
        Instruction::LoadVia { reg } if reg.index_u8() != 0 => encode_one(reg.index_u8(), out),
        Instruction::LoadVia { .. } => Err(EncodeError::InvalidOperands),
        Instruction::Increment { reg } => encode_one(0x10 | reg.index_u8(), out),
        Instruction::Decrement { reg } => encode_one(0x20 | reg.index_u8(), out),
        Instruction::Branch { target } => encode_two(Opcode::Branch as u8, target, out),
        Instruction::BranchQ { expected, target } => {
            encode_two(if expected { 0x31 } else { 0x39 }, target, out)
        }
        Instruction::BranchZero { expected, target } => {
            encode_two(if expected { 0x32 } else { 0x3A }, target, out)
        }
        Instruction::BranchDataFlag { expected, target } => {
            encode_two(if expected { 0x33 } else { 0x3B }, target, out)
        }
        Instruction::BranchExternalFlag {
            flag,
            expected,
            target,
        } => {
            let base = if expected { 0x33 } else { 0x3B };
            encode_two(base + flag.index_u8(), target, out)
        }
        Instruction::ShortSkip { filler } => encode_two(Opcode::ShortSkip as u8, filler, out),
        Instruction::LoadAdvance { reg } => encode_one(0x40 | reg.index_u8(), out),
        Instruction::Store { reg } => encode_one(0x50 | reg.index_u8(), out),
        Instruction::Irx => encode_one(Opcode::Irx as u8, out),
        Instruction::Output { port } if (1..=7).contains(&port) => encode_one(0x60 | port, out),
        Instruction::Input { port } if (1..=7).contains(&port) => encode_one(0x68 | port, out),
        Instruction::Output { .. } | Instruction::Input { .. } => Err(EncodeError::InvalidOperands),
        Instruction::Reserved68 => encode_one(Opcode::Reserved68 as u8, out),
        Instruction::Return => encode_one(Opcode::Return as u8, out),
        Instruction::DisableInterrupt => encode_one(Opcode::DisableInterrupt as u8, out),
        Instruction::LoadViaXAdvance => encode_one(Opcode::LoadViaXAdvance as u8, out),
        Instruction::StoreViaXDecrement => encode_one(Opcode::StoreViaXDecrement as u8, out),
        Instruction::AddWithCarry => encode_one(Opcode::AddWithCarry as u8, out),
        Instruction::SubtractDWithBorrow => encode_one(Opcode::SubtractDWithBorrow as u8, out),
        Instruction::ShiftRightWithCarry => encode_one(Opcode::ShiftRightWithCarry as u8, out),
        Instruction::SubtractMemoryWithBorrow => {
            encode_one(Opcode::SubtractMemoryWithBorrow as u8, out)
        }
        Instruction::Save => encode_one(Opcode::Save as u8, out),
        Instruction::Mark => encode_one(Opcode::Mark as u8, out),
        Instruction::ResetQ => encode_one(Opcode::ResetQ as u8, out),
        Instruction::SetQ => encode_one(Opcode::SetQ as u8, out),
        Instruction::AddWithCarryImmediate { value } => {
            encode_two(Opcode::AddWithCarryImmediate as u8, value, out)
        }
        Instruction::SubtractDWithBorrowImmediate { value } => {
            encode_two(Opcode::SubtractDWithBorrowImmediate as u8, value, out)
        }
        Instruction::ShiftLeftWithCarry => encode_one(Opcode::ShiftLeftWithCarry as u8, out),
        Instruction::SubtractMemoryWithBorrowImmediate { value } => {
            encode_two(Opcode::SubtractMemoryWithBorrowImmediate as u8, value, out)
        }
        Instruction::GetLow { reg } => encode_one(0x80 | reg.index_u8(), out),
        Instruction::GetHigh { reg } => encode_one(0x90 | reg.index_u8(), out),
        Instruction::PutLow { reg } => encode_one(0xA0 | reg.index_u8(), out),
        Instruction::PutHigh { reg } => encode_one(0xB0 | reg.index_u8(), out),
        Instruction::LongBranch { condition, target } => {
            encode_long(long_branch_opcode(condition), target, out)
        }
        Instruction::NoOperation => encode_one(Opcode::NoOperation as u8, out),
        Instruction::LongSkip { condition } => encode_one(long_skip_opcode(condition), out),
        Instruction::SetP { reg } => encode_one(0xD0 | reg.index_u8(), out),
        Instruction::SetX { reg } => encode_one(0xE0 | reg.index_u8(), out),
        Instruction::LoadViaX => encode_one(Opcode::LoadViaX as u8, out),
        Instruction::Or => encode_one(Opcode::Or as u8, out),
        Instruction::And => encode_one(Opcode::And as u8, out),
        Instruction::Xor => encode_one(Opcode::Xor as u8, out),
        Instruction::Add => encode_one(Opcode::Add as u8, out),
        Instruction::SubtractDNoBorrow => encode_one(Opcode::SubtractDNoBorrow as u8, out),
        Instruction::ShiftRight => encode_one(Opcode::ShiftRight as u8, out),
        Instruction::SubtractMemoryNoBorrow => {
            encode_one(Opcode::SubtractMemoryNoBorrow as u8, out)
        }
        Instruction::LoadImmediate { value } => encode_two(Opcode::LoadImmediate as u8, value, out),
        Instruction::OrImmediate { value } => encode_two(Opcode::OrImmediate as u8, value, out),
        Instruction::AndImmediate { value } => encode_two(Opcode::AndImmediate as u8, value, out),
        Instruction::XorImmediate { value } => encode_two(Opcode::XorImmediate as u8, value, out),
        Instruction::AddImmediate { value } => encode_two(Opcode::AddImmediate as u8, value, out),
        Instruction::SubtractDImmediateNoBorrow { value } => {
            encode_two(Opcode::SubtractDImmediateNoBorrow as u8, value, out)
        }
        Instruction::ShiftLeft => encode_one(Opcode::ShiftLeft as u8, out),
        Instruction::SubtractMemoryNoBorrowImmediate { value } => {
            encode_two(Opcode::SubtractMemoryNoBorrowImmediate as u8, value, out)
        }
    }
}

fn long_branch_opcode(condition: LongBranchCondition) -> u8 {
    match condition {
        LongBranchCondition::Always => 0xC0,
        LongBranchCondition::Q => 0xC1,
        LongBranchCondition::Zero => 0xC2,
        LongBranchCondition::DataFlag => 0xC3,
        LongBranchCondition::NotQ => 0xC9,
        LongBranchCondition::NotZero => 0xCA,
        LongBranchCondition::NotDataFlag => 0xCB,
    }
}

fn long_skip_opcode(condition: LongSkipCondition) -> u8 {
    match condition {
        LongSkipCondition::NotQ => 0xC5,
        LongSkipCondition::NotZero => 0xC6,
        LongSkipCondition::NotDataFlag => 0xC7,
        LongSkipCondition::Always => 0xC8,
        LongSkipCondition::InterruptEnabled => 0xCC,
        LongSkipCondition::Q => 0xCD,
        LongSkipCondition::Zero => 0xCE,
        LongSkipCondition::DataFlag => 0xCF,
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

fn encode_long(first: u8, target: u16, out: &mut [u8]) -> Result<usize, EncodeError> {
    if out.len() < 3 {
        return Err(EncodeError::BufferTooSmall);
    }
    let [high, low] = target.to_be_bytes();
    out[0] = first;
    out[1] = high;
    out[2] = low;
    Ok(3)
}
