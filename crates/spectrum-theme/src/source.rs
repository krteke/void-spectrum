//! Extension traits for generated theme token sources.

/// A source that can provide typed token values by contract path.
pub trait TokenSource {
    /// Error produced while reading a token value.
    type Error;

    /// Reads a token value with the type requested by the generated contract.
    fn token<T>(&self, path: &str) -> Result<T, Self::Error>
    where
        T: ThemeValue<Self>,
        Self: Sized,
    {
        T::read(self, path)
    }

    /// Returns whether an error means "token not present".
    ///
    /// Generated state inheritance uses this to fall back to parent states.
    fn is_missing(error: &Self::Error) -> bool {
        let _ = error;
        false
    }
}

/// A typed theme value that can be read from a [`TokenSource`].
pub trait ThemeValue<S: TokenSource>: Sized {
    /// Reads `Self` from `source` at `path`.
    fn read(source: &S, path: &str) -> Result<Self, S::Error>;
}

/// Reads the first present token from `paths`.
///
/// Missing-token errors fall through to the next path. Other errors are
/// returned immediately.
pub fn read_inherited<T, S, const N: usize>(source: &S, paths: [&str; N]) -> Result<T, S::Error>
where
    T: ThemeValue<S>,
    S: TokenSource,
{
    let mut missing = None;
    for path in paths {
        match source.token::<T>(path) {
            Ok(value) => return Ok(value),
            Err(error) if S::is_missing(&error) => {
                missing.get_or_insert(error);
            }
            Err(error) => return Err(error),
        }
    }
    Err(missing.expect("inherited token lookup has at least one path"))
}
