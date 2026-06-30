use std::{env, path::PathBuf};

/// Errors produced by build-time theme code generation.
#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    /// The generator could not read the source theme file.
    #[error("failed to read theme source '{}': {source}", path.display())]
    ReadSource {
        /// Source file path.
        path: PathBuf,
        /// I/O failure.
        source: std::io::Error,
    },
    /// The generator could not write the generated Rust file.
    #[error("failed to write generated theme code '{}': {source}", path.display())]
    WriteOutput {
        /// Output file path.
        path: PathBuf,
        /// I/O failure.
        source: std::io::Error,
    },
    /// The values file is not valid contract-aware TOML.
    #[error("failed to parse theme source '{}': {source}", path.display())]
    ParseToml {
        /// Source file path.
        path: PathBuf,
        /// TOML parse failure.
        source: toml::de::Error,
    },
    /// The generated Rust contract contains an invalid Rust identifier or type.
    #[error("invalid generated Rust contract: {0}")]
    InvalidContract(syn::Error),
    /// `OUT_DIR` is unavailable for a build-script generation call.
    #[error("OUT_DIR is not set: {0}")]
    MissingOutDir(env::VarError),
}
