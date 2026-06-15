//! Compile-fail tests for file-driven token contracts.

#![cfg(feature = "macros")]

#[test]
fn rejects_invalid_embedded_themes() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/*.rs");
}
