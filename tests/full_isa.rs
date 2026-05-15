//! Full-opcode coverage for the RCA CDP1802 ISA crate.

use sw_cdp1802_isa::{
    Addr, Cdp1802, ExternalFlag, Instruction, LongBranchCondition, LongSkipCondition, Reg,
};
use sw_isa_core::Architecture;

#[test]
fn every_opcode_decodes_and_round_trips() {
    for opcode in 0x00u8..=0xFF {
        let bytes = [opcode, 0x12, 0x34];
        let (insn, len) = Cdp1802::decode(&bytes, Addr(0))
            .unwrap_or_else(|err| panic!("decode failed for opcode 0x{opcode:02x}: {err:?}"));
        assert_eq!(
            len,
            expected_len(opcode),
            "length for opcode 0x{opcode:02x}"
        );

        let mut out = [0u8; 3];
        let n = Cdp1802::encode(&insn, &mut out)
            .unwrap_or_else(|err| panic!("encode failed for opcode 0x{opcode:02x}: {err:?}"));
        assert_eq!(n, len, "round-trip length for opcode 0x{opcode:02x}");
        assert_eq!(
            &out[..n],
            &bytes[..len],
            "round-trip bytes for opcode 0x{opcode:02x}"
        );

        let mut text = String::new();
        Cdp1802::disassemble(&insn, &mut text)
            .unwrap_or_else(|err| panic!("disassemble failed for opcode 0x{opcode:02x}: {err:?}"));
        assert!(
            !text.is_empty(),
            "disassembly text for opcode 0x{opcode:02x}"
        );
    }
}

#[test]
fn representative_full_instruction_encodings() {
    let cases: &[(&str, Instruction, &[u8])] = &[
        (
            "ldn r1",
            Instruction::LoadVia {
                reg: Reg::new_masked(1),
            },
            &[0x01],
        ),
        (
            "dec rf",
            Instruction::Decrement {
                reg: Reg::new_masked(0x0f),
            },
            &[0x2f],
        ),
        (
            "bq",
            Instruction::BranchQ {
                expected: true,
                target: 0x80,
            },
            &[0x31, 0x80],
        ),
        (
            "bnz",
            Instruction::BranchZero {
                expected: false,
                target: 0x81,
            },
            &[0x3a, 0x81],
        ),
        (
            "bnf",
            Instruction::BranchDataFlag {
                expected: false,
                target: 0x82,
            },
            &[0x3b, 0x82],
        ),
        (
            "skp",
            Instruction::ShortSkip { filler: 0x00 },
            &[0x38, 0x00],
        ),
        (
            "lda r2",
            Instruction::LoadAdvance {
                reg: Reg::new_masked(2),
            },
            &[0x42],
        ),
        ("irx", Instruction::Irx, &[0x60]),
        ("reserved68", Instruction::Reserved68, &[0x68]),
        ("ret", Instruction::Return, &[0x70]),
        ("dis", Instruction::DisableInterrupt, &[0x71]),
        ("ldxa", Instruction::LoadViaXAdvance, &[0x72]),
        ("stxd", Instruction::StoreViaXDecrement, &[0x73]),
        ("adc", Instruction::AddWithCarry, &[0x74]),
        ("sdb", Instruction::SubtractDWithBorrow, &[0x75]),
        ("shrc", Instruction::ShiftRightWithCarry, &[0x76]),
        ("smb", Instruction::SubtractMemoryWithBorrow, &[0x77]),
        ("sav", Instruction::Save, &[0x78]),
        ("mark", Instruction::Mark, &[0x79]),
        (
            "adci",
            Instruction::AddWithCarryImmediate { value: 0x44 },
            &[0x7c, 0x44],
        ),
        (
            "sdbi",
            Instruction::SubtractDWithBorrowImmediate { value: 0x45 },
            &[0x7d, 0x45],
        ),
        ("shlc", Instruction::ShiftLeftWithCarry, &[0x7e]),
        (
            "smbi",
            Instruction::SubtractMemoryWithBorrowImmediate { value: 0x46 },
            &[0x7f, 0x46],
        ),
        (
            "ghi r3",
            Instruction::GetHigh {
                reg: Reg::new_masked(3),
            },
            &[0x93],
        ),
        (
            "lbr",
            Instruction::LongBranch {
                condition: LongBranchCondition::Always,
                target: 0x1234,
            },
            &[0xc0, 0x12, 0x34],
        ),
        (
            "lbnz",
            Instruction::LongBranch {
                condition: LongBranchCondition::NotZero,
                target: 0x2345,
            },
            &[0xca, 0x23, 0x45],
        ),
        ("nop", Instruction::NoOperation, &[0xc4]),
        (
            "lsie",
            Instruction::LongSkip {
                condition: LongSkipCondition::InterruptEnabled,
            },
            &[0xcc],
        ),
        (
            "lsdf",
            Instruction::LongSkip {
                condition: LongSkipCondition::DataFlag,
            },
            &[0xcf],
        ),
        (
            "sep r4",
            Instruction::SetP {
                reg: Reg::new_masked(4),
            },
            &[0xd4],
        ),
        ("ldx", Instruction::LoadViaX, &[0xf0]),
        ("or", Instruction::Or, &[0xf1]),
        ("and", Instruction::And, &[0xf2]),
        ("xor", Instruction::Xor, &[0xf3]),
        ("sd", Instruction::SubtractDNoBorrow, &[0xf5]),
        ("shr", Instruction::ShiftRight, &[0xf6]),
        ("sm", Instruction::SubtractMemoryNoBorrow, &[0xf7]),
        (
            "ori",
            Instruction::OrImmediate { value: 0x11 },
            &[0xf9, 0x11],
        ),
        (
            "ani",
            Instruction::AndImmediate { value: 0x22 },
            &[0xfa, 0x22],
        ),
        (
            "xri",
            Instruction::XorImmediate { value: 0x33 },
            &[0xfb, 0x33],
        ),
        (
            "sdi",
            Instruction::SubtractDImmediateNoBorrow { value: 0x44 },
            &[0xfd, 0x44],
        ),
        (
            "smi",
            Instruction::SubtractMemoryNoBorrowImmediate { value: 0x55 },
            &[0xff, 0x55],
        ),
    ];

    for (name, insn, bytes) in cases {
        let mut out = [0u8; 3];
        let n = Cdp1802::encode(insn, &mut out)
            .unwrap_or_else(|err| panic!("encode failed for {name}: {err:?}"));
        assert_eq!(&out[..n], *bytes, "{name}: encoded bytes");

        let (decoded, m) = Cdp1802::decode(bytes, Addr(0))
            .unwrap_or_else(|err| panic!("decode failed for {name}: {err:?}"));
        assert_eq!(m, bytes.len(), "{name}: decoded length");
        assert_eq!(decoded, *insn, "{name}: decoded instruction");
    }
}

#[test]
fn ldn_r0_is_invalid_because_opcode_zero_is_idl() {
    assert_eq!(
        Cdp1802::decode(&[0x00], Addr(0)).unwrap().0,
        Instruction::Idle
    );
    assert!(
        Cdp1802::encode(
            &Instruction::LoadVia {
                reg: Reg::new_masked(0),
            },
            &mut [0]
        )
        .is_err()
    );
}

#[test]
fn external_flag_branch_family_still_round_trips_all_flags() {
    for index in 1..=4 {
        let flag = ExternalFlag::new(index).unwrap();
        let b = Instruction::BranchExternalFlag {
            flag,
            expected: true,
            target: 0xaa,
        };
        let bn = Instruction::BranchExternalFlag {
            flag,
            expected: false,
            target: 0xbb,
        };
        assert_round_trip(&b, &[0x33 + index, 0xaa]);
        assert_round_trip(&bn, &[0x3b + index, 0xbb]);
    }
}

fn assert_round_trip(insn: &Instruction, bytes: &[u8]) {
    let mut out = [0u8; 3];
    let n = Cdp1802::encode(insn, &mut out).unwrap();
    assert_eq!(&out[..n], bytes);
    let (decoded, m) = Cdp1802::decode(bytes, Addr(0)).unwrap();
    assert_eq!(m, bytes.len());
    assert_eq!(decoded, *insn);
}

fn expected_len(opcode: u8) -> usize {
    match opcode {
        0x30..=0x3f | 0x7c | 0x7d | 0x7f | 0xf8..=0xfd | 0xff => 2,
        0xc0..=0xc3 | 0xc9..=0xcb => 3,
        _ => 1,
    }
}
