# sw-cdp1802-isa

RCA CDP1802 ISA description: opcodes, encoding, decoding, disassembly.

## Status

`0.1.0` defines the full RCA CDP1802 opcode map for decode, encode, and
disassembly. Tests include exhaustive decode coverage over `0x00..=0xff` and
representative encode/decode round trips for every instruction family.

Assembler and emulator crates consume this shared ISA surface in follow-up
saga steps.

## Sibling layout

Cross-crate deps assume sibling clones at
`~/github/sw-langtools/<framework-crate>` and
`~/github/<host-org>/sw-cdp1802-<role>`. See
[`gen-isa/docs/decisions.md`](https://github.com/sw-vibe-coding/gen-isa/blob/main/docs/decisions.md)
Sec 1 for the full org map.

## License

MIT.
