//! Reference vectors for the CDP1802 demo subset.

use sw_cdp1802_isa::{Addr, Cdp1802, Instruction, Reg};
use sw_isa_core::Architecture;

const DEMO_BYTES: &[u8] = &[
    0xF8, 0x20, 0xB1, 0xF8, 0x00, 0xA1, 0xF8, 0x42, 0x51, 0x11, 0xF8, 0x43, 0x51, 0x11, 0xF8, 0x44,
    0x51, 0x30, 0x13, 0x00,
];

const DEMO_INSTRUCTIONS: &[Instruction] = &[
    Instruction::LoadImmediate { value: 0x20 },
    Instruction::PutHigh {
        reg: Reg::new_masked(1),
    },
    Instruction::LoadImmediate { value: 0x00 },
    Instruction::PutLow {
        reg: Reg::new_masked(1),
    },
    Instruction::LoadImmediate { value: 0x42 },
    Instruction::Store {
        reg: Reg::new_masked(1),
    },
    Instruction::Increment {
        reg: Reg::new_masked(1),
    },
    Instruction::LoadImmediate { value: 0x43 },
    Instruction::Store {
        reg: Reg::new_masked(1),
    },
    Instruction::Increment {
        reg: Reg::new_masked(1),
    },
    Instruction::LoadImmediate { value: 0x44 },
    Instruction::Store {
        reg: Reg::new_masked(1),
    },
    Instruction::Branch { target: 0x13 },
    Instruction::Idle,
];

#[test]
fn demo_subset_exact_encodings() {
    let cases: &[(&str, Instruction, &[u8])] = &[
        ("idl", Instruction::Idle, &[0x00]),
        (
            "inc r1",
            Instruction::Increment {
                reg: Reg::new_masked(1),
            },
            &[0x11],
        ),
        (
            "br 0x13",
            Instruction::Branch { target: 0x13 },
            &[0x30, 0x13],
        ),
        (
            "str r1",
            Instruction::Store {
                reg: Reg::new_masked(1),
            },
            &[0x51],
        ),
        (
            "plo r1",
            Instruction::PutLow {
                reg: Reg::new_masked(1),
            },
            &[0xA1],
        ),
        (
            "phi r1",
            Instruction::PutHigh {
                reg: Reg::new_masked(1),
            },
            &[0xB1],
        ),
        (
            "ldi 0x42",
            Instruction::LoadImmediate { value: 0x42 },
            &[0xF8, 0x42],
        ),
    ];

    for (name, insn, bytes) in cases {
        let mut out = [0u8; 2];
        let n = Cdp1802::encode(insn, &mut out)
            .unwrap_or_else(|e| panic!("encode failed for {name}: {e:?}"));
        assert_eq!(&out[..n], *bytes, "{name}: encoded bytes");

        let (decoded, m) = Cdp1802::decode(bytes, Addr(0))
            .unwrap_or_else(|e| panic!("decode failed for {name}: {e:?}"));
        assert_eq!(m, bytes.len(), "{name}: decoded length");
        assert_eq!(decoded, *insn, "{name}: decoded instruction");
    }
}

#[test]
fn demo_program_decodes_to_expected_instruction_stream() {
    let mut offset = 0usize;
    let mut decoded = Vec::new();
    while offset < DEMO_BYTES.len() {
        let (insn, n) = Cdp1802::decode(&DEMO_BYTES[offset..], Addr(offset as u32))
            .unwrap_or_else(|e| panic!("decode failed at {offset:#x}: {e:?}"));
        decoded.push(insn);
        offset += n;
    }
    assert_eq!(&decoded[..], DEMO_INSTRUCTIONS);
}

#[test]
fn demo_program_encodes_to_expected_bytes() {
    let mut encoded = [0u8; DEMO_BYTES.len()];
    let mut offset = 0usize;
    for insn in DEMO_INSTRUCTIONS {
        let n = Cdp1802::encode(insn, &mut encoded[offset..])
            .unwrap_or_else(|e| panic!("encode failed for {insn:?}: {e:?}"));
        offset += n;
    }
    assert_eq!(offset, DEMO_BYTES.len());
    assert_eq!(&encoded, DEMO_BYTES);
}

#[test]
fn demo_program_round_trips_instruction_by_instruction() {
    let mut offset = 0usize;
    while offset < DEMO_BYTES.len() {
        let (insn, n) = Cdp1802::decode(&DEMO_BYTES[offset..], Addr(offset as u32)).unwrap();
        let mut out = [0u8; 2];
        let m = Cdp1802::encode(&insn, &mut out).unwrap();
        assert_eq!(m, n, "round-trip length at {offset:#x}");
        assert_eq!(&out[..m], &DEMO_BYTES[offset..offset + n]);
        offset += n;
    }
}
