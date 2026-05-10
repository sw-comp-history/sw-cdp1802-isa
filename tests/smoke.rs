//! Smoke test: crate compiles and the marker type is constructible.

#[test]
fn marker_constructs() {
    let _ = sw_cdp1802_isa::Cdp1802;
}
