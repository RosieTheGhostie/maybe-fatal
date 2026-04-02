use core::fmt::Display;

use sealed::sealed;

/// The default type of a [`DiagnosticCode`](super::DiagnosticCode)'s discriminant.
pub type DefaultDiscriminant = u8;

/// A discriminant for a [`DiagnosticCode`](super::DiagnosticCode).
///
/// # Note to Consumers
///
/// This trait is **sealed**; it cannot be implemented outside the [`maybe_fatal`](crate) crate.
#[sealed]
pub trait Discriminant: Display {
    /// The max number of digits in the discriminant.
    const MAX_N_DIGITS: usize;
}

macro_rules! impl_Discriminant {
    [$($ty:ty),* $(,)?] => {
        $(
            #[sealed]
            impl Discriminant for $ty {
                const MAX_N_DIGITS: usize = 1 + Self::MAX.ilog10() as usize;
            }
        )*
    };
}

impl_Discriminant![u8, u16, u32, u64, u128];

// Asserts that `DefaultDiscriminant` implements `Discriminant`.
const _: () = {
    let _ = <DefaultDiscriminant as Discriminant>::MAX_N_DIGITS;
};
