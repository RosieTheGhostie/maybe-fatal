//! Various traits for working with [`Diagnostic`]s.
//!
//! The following traits can be derived if the "derive" feature is enabled:
//!
//! - [`Diagnose`]
//! - [`PartialDiagnose`]
//! - [`DiagnosticGroup`]

use sealed::sealed;

use crate::{ColorPalette, Diagnostic, DiagnosticCode, code};

/// Used to indicate that the implementing type does not imply a
/// [diagnostic severity](crate::DiagnosticSeverity) of [`Error`](crate::DiagnosticSeverity::Error).
pub trait Lenient {}

impl<S, T> Lenient for (S, T)
where
    S: ariadne::Span,
    T: Lenient,
{
}

/// A trait for things that can be transformed into a [`Diagnostic`] that is ready to report.
///
/// High-level error types should implement this.
///
/// # Tip
///
/// This trait can be derived if the "derive" feature is enabled.
pub trait Diagnose<S, D = code::DefaultDiscriminant> {
    /// Transforms `self` into a [`Diagnostic`].
    ///
    /// Implementers can choose to make use of the provided color palette if they wish.
    fn diagnose(self, colors: &ColorPalette) -> Diagnostic<S, D>;
}

impl<T, S, D> Diagnose<S, D> for (S, T)
where
    T: DiagnosticGroup<D> + PartialDiagnose<S, D> + Sized,
{
    fn diagnose(self, colors: &ColorPalette) -> Diagnostic<S, D> {
        let (span, group_member) = self;
        let diagnostic = group_member.make_diagnostic(span);

        group_member.partial_diagnose(diagnostic, colors)
    }
}

/// A trait for things that can finish an existing [diagnostic](Diagnostic), but not start one.
///
/// This is primarily for error variants that don't know their own span and/or error code.
///
/// # Tip
///
/// This trait can be derived if the "derive" feature is enabled.
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
/// # Tip
///
/// This trait can be derived if the "derive" feature is enabled.
///
/// [`thiserror::Error`]: https://docs.rs/thiserror/latest/thiserror/derive.Error.html
pub trait DiagnosticGroup<D = code::DefaultDiscriminant> {
    /// Builds the diagnostic message corresponding to `self`.
    fn message(&self) -> String;

    /// Gets the [`DiagnosticCode`] corresponding to `self`.
    fn diagnostic_code(&self) -> DiagnosticCode<D>;
}

/// Provides a number of extension methods for [diagnostic group](DiagnosticGroup)s.
///
/// # Note to Consumers
///
/// This trait is **sealed**; it cannot be implemented outside the [`maybe_fatal`](crate) crate. If
/// you want your type to implement this trait, implement [`DiagnosticGroup`].
///
/// The reason for this, by the way, is to provide more guarantees about what each of the methods
/// do. By sealing the trait, this crate can guarantee that these methods do what they say they do.
#[sealed]
pub trait DiagnosticGroupExt<D = code::DefaultDiscriminant>: DiagnosticGroup<D> {
    /// Makes a diagnostic with the given span using information from `self`.
    ///
    /// # Note
    ///
    /// The resulting diagnostic will _only_ have a message, span, and code. If `Self` implements
    /// [`PartialDiagnose`] as well (which it probably should), you should prefer to call
    /// [`into_diagnostic`](DiagnosticGroupExt::into_diagnostic) instead.
    fn make_diagnostic<S>(&self, span: S) -> Diagnostic<S, D> {
        Diagnostic::new::<Self>(self, span)
    }

    /// Transforms `self` into a [`Diagnostic`] with the given span.
    ///
    /// The color palette will be forwarded to `self`'s implementation of
    /// [`PartialDiagnose::partial_diagnose`].
    fn into_diagnostic<S>(self, span: S, colors: &ColorPalette) -> Diagnostic<S, D>
    where
        Self: PartialDiagnose<S, D> + Sized,
    {
        (span, self).diagnose(colors)
    }
}

#[sealed]
impl<T, D> DiagnosticGroupExt<D> for T where T: DiagnosticGroup<D> {}
