use std::{env, path::PathBuf};

use spectrum_resolver::ResolveError;

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
    /// The theme file is not valid TOML for the supported schema.
    #[error("failed to parse theme source '{}': {source}", path.display())]
    ParseToml {
        /// Source file path.
        path: PathBuf,
        /// TOML parse failure.
        source: toml::de::Error,
    },
    /// The theme file could not be resolved into concrete token values.
    #[error("failed to resolve theme source '{}': {source}", path.display())]
    Resolve {
        /// Source file path.
        path: PathBuf,
        /// Resolver failure.
        source: ResolveError,
    },
    /// The generated Rust contract contains an invalid Rust identifier or type.
    #[error("invalid generated Rust contract: {0}")]
    InvalidContract(syn::Error),
    /// `OUT_DIR` is unavailable for a build-script generation call.
    #[error("OUT_DIR is not set: {0}")]
    MissingOutDir(env::VarError),
}
