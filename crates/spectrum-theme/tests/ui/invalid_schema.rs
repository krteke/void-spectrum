use spectrum_theme::include_theme_tokens;

include_theme_tokens! {
    struct InvalidSchema;
    source = "invalid_schema.toml";
    format = toml;
}

fn main() {}
