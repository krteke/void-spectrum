/// Errors produced while resolving a theme specification.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ResolveError {
    /// Two shadow entries declare the same token path.
    #[error("duplicate shadow token '{token}'")]
    DuplicateShadow {
        /// Repeated shadow token path.
        token: String,
    },
    /// A shadow layer violates the core value contract.
    #[error("invalid shadow token '{token}': {source}")]
    InvalidShadow {
        /// Invalid shadow token path.
        token: String,
        /// Core validation failure.
        source: spectrum_core::ShadowError,
    },
    /// A token points to a token that is not directly resolvable.
    #[error("token '{token}' references unresolved token '{reference}'")]
    UnresolvedReference {
        /// Token being resolved.
        token: String,
        /// Referenced token path.
        reference: String,
    },
    /// Token references form a cycle.
    #[error("circular token reference: {}", chain.join(" -> "))]
    CircularReference {
        /// Ordered reference chain with the first token repeated at the end.
        chain: Vec<String>,
    },
}
