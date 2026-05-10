//! Branch range constants and reachability helpers.

pub const BRANCH_PAGE_SIZE_BYTES: usize = 256;
pub const BRANCH_TARGET_MIN: u16 = 0x00;
pub const BRANCH_TARGET_MAX: u16 = 0xFF;
pub const MAX_INSTRUCTION_BYTES: usize = 2;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExternalFlag {
    Ef1,
    Ef2,
    Ef3,
    Ef4,
}

impl ExternalFlag {
    pub const fn new(index: u8) -> Option<Self> {
        match index {
            1 => Some(Self::Ef1),
            2 => Some(Self::Ef2),
            3 => Some(Self::Ef3),
            4 => Some(Self::Ef4),
            _ => None,
        }
    }

    pub const fn index_u8(self) -> u8 {
        match self {
            Self::Ef1 => 1,
            Self::Ef2 => 2,
            Self::Ef3 => 3,
            Self::Ef4 => 4,
        }
    }
}

/// CDP1802 short branch opcodes keep the current high address byte and
/// replace only the low byte with the encoded target.
pub fn can_branch_within_page(from: u16, to: u16) -> bool {
    (from & 0xFF00) == (to & 0xFF00)
}
