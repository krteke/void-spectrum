/// Errors produced while resolving a theme specification.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ResolveError {
    /// A color token points to a token that is not directly resolvable.
    #[error("color token '{token}' references unresolved token '{reference}'")]
    UnresolvedReference {
        /// Token being resolved.
        token: String,
        /// Referenced token path.
        reference: String,
    },
}
