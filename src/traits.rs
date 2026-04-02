use sealed::sealed;

use crate::{ColorPalette, Diagnostic, DiagnosticCode, code};

/// Used to indicate that the implementing type does not imply a
/// [diagnostic severity](crate::DiagnosticSeverity) of [`Error`](crate::DiagnosticSeverity::Error).
///
/// This is not used directly by the [`maybe_fatal`](crate) crate; instead, it is intended to be a
/// utility for downstream crates creating [`Diagnostic`]s.
pub trait Lenient {}

/// A trait for things that can be transformed into a [`Diagnostic`] that is ready to report.
///
/// High-level error types should implement this.
pub trait Diagnose<S, D = code::DefaultDiscriminant> {
    /// Transforms `self` into a [`Diagnostic`].
    ///
    /// Implementers can choose to make use of the provided color palette if they wish.
    fn diagnose(self, colors: &ColorPalette) -> Diagnostic<S, D>;
}

/// A trait for things that can finish an existing [diagnostic](Diagnostic), but not start one.
///
/// This is primarily for error variants that don't know their own span and/or error code.
pub trait PartialDiagnose<S, D = code::DefaultDiscriminant> {
    /// Adds diagnostic information from `self` onto `diagnostic`, returning the result.
    fn partial_diagnose(
        self,
        diagnostic: Diagnostic<S, D>,
        colors: &ColorPalette,
    ) -> Diagnostic<S, D>;
}

/// A trait for things that represent a group of diagnostics.
///
/// Error `enum`s like those one would annotate with [`thiserror::Error`] should typically implement
/// this.
///
/// [`thiserror::Error`]: https://docs.rs/thiserror/latest/thiserror/derive.Error.html
pub trait DiagnosticGroup<D = code::DefaultDiscriminant> {
    fn diagnostic_code(&self) -> DiagnosticCode<D>;
}

#[sealed]
pub trait DiagnosticGroupExt<D = code::DefaultDiscriminant>: DiagnosticGroup<D> {
    fn make_diagnostic<S>(&self, span: S) -> Diagnostic<S, D> {
        Diagnostic::new::<Self>(self, span)
    }
}

#[sealed]
impl<T, D> DiagnosticGroupExt<D> for T where T: DiagnosticGroup<D> {}

/// A trait for things that can resolve diagnostic messages from their codes.
///
/// Implementing types will typically be zero-sized, as they have no real state to track.
pub trait DiagnosticMessageResolver<D = code::DefaultDiscriminant> {
    fn message(code: &DiagnosticCode<D>) -> Option<&'static str>;
}
