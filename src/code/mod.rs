pub use discriminant::{DefaultDiscriminant, Discriminant};

mod discriminant;

use core::{
    fmt::{self, Display, Formatter},
    str::Utf8Error,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticCode<D = DefaultDiscriminant> {
    prefix: [u8; 3],
    pub discriminant: D,
}

impl<D> DiagnosticCode<D> {
    pub fn new(prefix: [u8; 3], discriminant: D) -> Result<Self, Utf8Error> {
        match str::from_utf8(&prefix) {
            Ok(_) => Ok(unsafe { Self::new_unchecked(prefix, discriminant) }),
            Err(err) => Err(err),
        }
    }

    pub unsafe fn new_unchecked(prefix: [u8; 3], discriminant: D) -> Self {
        Self {
            prefix,
            discriminant,
        }
    }

    pub const fn prefix_bytes(&self) -> &[u8; 3] {
        &self.prefix
    }

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
