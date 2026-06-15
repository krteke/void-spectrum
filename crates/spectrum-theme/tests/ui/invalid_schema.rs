use spectrum_theme::include_theme_tokens;

include_theme_tokens! {
    struct InvalidSchema;
    source = include_str!("invalid_schema.toml");
    format = toml;
}

fn main() {}
