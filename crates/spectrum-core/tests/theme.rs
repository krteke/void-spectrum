//! Tests for core theme value types.

use spectrum_core::ThemeMode;

#[test]
fn theme_mode_defaults_to_dark_and_has_stable_spelling() {
    assert_eq!(ThemeMode::default(), ThemeMode::Dark);
    assert_eq!(ThemeMode::Dark.to_string(), "dark");
    assert_eq!(ThemeMode::Light.to_string(), "light");
}
