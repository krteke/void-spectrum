//! Workspace skeleton smoke tests.

use spectrum_core as _;
use spectrum_palette as _;
use spectrum_resolver as _;
use spectrum_schema as _;
use spectrum_theme as _;

#[test]
fn facade_package_uses_the_expected_name() {
    assert_eq!(env!("CARGO_PKG_NAME"), "spectrum-theme");
}
