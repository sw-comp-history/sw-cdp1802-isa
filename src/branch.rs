//! Branch range constants and reachability helpers.

pub const BRANCH_PAGE_SIZE_BYTES: usize = 256;
pub const BRANCH_TARGET_MIN: u16 = 0x00;
pub const BRANCH_TARGET_MAX: u16 = 0xFF;
pub const MAX_INSTRUCTION_BYTES: usize = 2;

/// CDP1802 short branch opcodes keep the current high address byte and
/// replace only the low byte with the encoded target.
pub fn can_branch_within_page(from: u16, to: u16) -> bool {
    (from & 0xFF00) == (to & 0xFF00)
}
