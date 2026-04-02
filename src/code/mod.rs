pub use discriminant::{DefaultDiscriminant, Discriminant};

mod discriminant;

use core::{
    fmt::{self, Display, Formatter},
    str::Utf8Error,
};

/// A code that identifies a given kind of diagnostic.
///
/// These are meant to be used by users as unchanging identifiers by which they can look up more
/// detailed information about the diagnostic.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticCode<D = DefaultDiscriminant> {
    /// A short prefix corresponding to the broad category the diagnostic belongs to.
    prefix: [u8; 3],

    /// The numeric discriminant.
    pub discriminant: D,
}

impl<D> DiagnosticCode<D> {
    /// Constructs a new [`DiagnosticCode`].
    ///
    /// # Errors
    ///
    /// If `prefix` is not valid UTF-8, an error variant will be returned.
    ///
    /// # See Also
    ///
    /// - [`new_unchecked`](Self::new_unchecked)
    pub fn new(prefix: [u8; 3], discriminant: D) -> Result<Self, Utf8Error> {
        match str::from_utf8(&prefix) {
            Ok(_) => Ok(unsafe { Self::new_unchecked(prefix, discriminant) }),
            Err(err) => Err(err),
        }
    }

    /// Constructs a new [`DiagnosticCode`].
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring `prefix` is valid UTF-8. For a safe alternative, see
    /// [`new`].
    ///
    /// # See Also
    ///
    /// - [`new`]
    ///
    /// [`new`]: Self::new
    pub const unsafe fn new_unchecked(prefix: [u8; 3], discriminant: D) -> Self {
        Self {
            prefix,
            discriminant,
        }
    }

    /// Gets this code's prefix as a slice of raw, UTF-8-encoded bytes.
    ///
    /// # See Also
    ///
    /// - [`prefix_str`](Self::prefix_str)
    pub const fn prefix_bytes(&self) -> &[u8; 3] {
        &self.prefix
    }

    /// Gets this code's prefix as a [string slice](str).
    ///
    /// # See Also
    ///
    /// - [`prefix_bytes`](Self::prefix_bytes)
    pub const fn prefix_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.prefix) }
    }
}

impl<D> Display for DiagnosticCode<D>
where
    D: Discriminant,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{:0width$}",
            self.prefix_str(),
            self.discriminant,
            width = D::MAX_N_DIGITS,
        )
    }
}
