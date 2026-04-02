use crate::{Diagnostic, DiagnosticSeverity, code, traits::DiagnosticMessageResolver};

/// A [`Diagnostic`] with an explicit [severity](DiagnosticSeverity).
///
/// This is constructed through the [`Diagnostic::classify`] method.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ClassifiedDiagnostic<S, D = code::DefaultDiscriminant> {
    /// The diagnostic being classified.
    pub(super) inner: Diagnostic<S, D>,

    /// The severity of the diagnostic.
    pub severity: DiagnosticSeverity,
}

impl<S, D> ClassifiedDiagnostic<S, D> {
    /// Changes this diagnostic's severity to [`Error`](DiagnosticSeverity::Error).
    pub const fn make_error(&mut self) -> &mut Self {
        self.severity = DiagnosticSeverity::Error;
        self
    }

    /// Changes this diagnostic's severity to [`Warning`](DiagnosticSeverity::Warning).
    pub const fn make_warning(&mut self) -> &mut Self {
        self.severity = DiagnosticSeverity::Warning;
        self
    }

    /// Changes this diagnostic's severity to [`Advice`](DiagnosticSeverity::Advice).
    pub const fn make_advice(&mut self) -> &mut Self {
        self.severity = DiagnosticSeverity::Advice;
        self
    }

    /// Reports this diagnostic using the given configuration.
    ///
    /// See the [`ariadne`] documentation for more details.
    pub fn report<Resolver, C>(self, config: ariadne::Config, cache: C) -> std::io::Result<()>
    where
        S: ariadne::Span,
        D: code::Discriminant,
        Resolver: DiagnosticMessageResolver<D>,
        C: ariadne::Cache<S::SourceId>,
    {
        self.inner
            .report_with::<Resolver, _>(self.severity, config, cache)
    }
}
