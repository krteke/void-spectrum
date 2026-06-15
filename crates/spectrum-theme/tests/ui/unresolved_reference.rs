use spectrum_theme::include_theme_tokens;

include_theme_tokens! {
    struct UnresolvedReference;
    source = include_str!("unresolved_reference.toml");
    format = toml;
}

fn main() {}
