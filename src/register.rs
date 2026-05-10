//! Register identifier and parser.

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reg(u8);

impl Reg {
    pub const fn new(index: u8) -> Option<Self> {
        if index < 16 { Some(Self(index)) } else { None }
    }

    pub const fn new_masked(index: u8) -> Self {
        Self(index & 0x0F)
    }

    pub const fn index_u8(self) -> u8 {
        self.0
    }

    pub fn name(self) -> &'static str {
        REGISTER_NAMES[self.0 as usize]
    }
}

impl sw_isa_core::register::RegisterId for Reg {
    fn index(self) -> u32 {
        self.0 as u32
    }

    fn name(self) -> &'static str {
        Reg::name(self)
    }
}

const REGISTER_NAMES: [&str; 16] = [
    "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12", "r13", "r14",
    "r15",
];

pub fn parse_register(s: &str) -> Option<Reg> {
    let rest = s.strip_prefix('r').or_else(|| s.strip_prefix('R'))?;
    let value = parse_decimal_u8(rest)?;
    Reg::new(value)
}

fn parse_decimal_u8(s: &str) -> Option<u8> {
    if s.is_empty() {
        return None;
    }
    let mut value: u16 = 0;
    for b in s.bytes() {
        if !b.is_ascii_digit() {
            return None;
        }
        value = value * 10 + (b - b'0') as u16;
        if value > u8::MAX as u16 {
            return None;
        }
    }
    Some(value as u8)
}
