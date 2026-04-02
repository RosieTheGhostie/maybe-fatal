#![doc = include_str!("../README.md")]

pub mod code;
pub mod prelude;
pub mod sink;
pub mod traits;

pub use ariadne::{Config, Label};

pub use classified::ClassifiedDiagnostic;
pub use color_palette::ColorPalette;
pub use context::Context;
pub use severity::DiagnosticSeverity;

mod classified;
mod color_palette;
mod context;
mod severity;

use core::borrow::Borrow;

use code::DiagnosticCode;
use traits::{DiagnosticGroup, DiagnosticMessageResolver};

/// A contextualized message meant to assist the user in diagnosing and resolving issues.
///
/// This is intended for use in things like parsers and compilers, but they can be used in any
/// text-based input processing.
///
/// Diagnostics are neutral by default; that is, they don't have an assigned
/// [severity](DiagnosticSeverity). To assign a severity, use the [`classify`](Self::classify)
/// method.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Diagnostic<S, D = code::DefaultDiscriminant> {
    /// The code identifying this kind of diagnostic.
    ///
    /// Diagnostics with identical codes are not necessarily equivalent. They may refer to different
    /// spans and have different contextual information.
    pub code: DiagnosticCode<D>,

    /// The span this diagnostic refers to.
    pub span: S,

    /// Any contextual information that may accompany the message.
    pub context_info: Vec<Context<S>>,
}

impl<S, D> Diagnostic<S, D> {
    /// Constructs a new [`Diagnostic`] from the provided member of a [`DiagnosticGroup`].
    pub fn new<T>(group_member: impl Borrow<T>, span: S) -> Self
    where
        T: DiagnosticGroup<D> + ?Sized,
    {
        Self {
            code: group_member.borrow().diagnostic_code(),
            span,
            context_info: Vec::new(),
        }
    }

    /// Labels a subspan of this diagnostic.
    ///
    /// See the documentation of [`Label`] for more details.
    pub fn with_label(mut self, label: Label<S>) -> Self {
        self.context_info.push(Context::Label(label));
        self
    }

    /// Sequentially labels several subspans of this diagnostic.
    ///
    /// See the documentation of [`Label`] for more details.
    pub fn with_labels(mut self, labels: impl IntoIterator<Item = Label<S>>) -> Self {
        self.context_info
            .extend(labels.into_iter().map(Context::Label));
        self
    }

    /// Adds a note to this diagnostic.
    pub fn with_note(mut self, note: impl ToString) -> Self {
        self.context_info.push(Context::new_note(note));
        self
    }

    /// Adds a help message to this diagnostic.
    pub fn with_help(mut self, help: impl ToString) -> Self {
        self.context_info.push(Context::new_help(help));
        self
    }

    /// Classifies this diagnostic under the given severity.
    pub const fn classify(self, severity: DiagnosticSeverity) -> ClassifiedDiagnostic<S, D> {
        ClassifiedDiagnostic {
            inner: self,
            severity,
        }
    }

    /// Reports this diagnostic using the given severity and configuration.
    ///
    /// See the [`ariadne`] documentation for more details.
    pub fn report_with<Resolver, C>(
        self,
        severity: DiagnosticSeverity,
        config: Config,
        cache: C,
    ) -> std::io::Result<()>
    where
        S: ariadne::Span,
        D: code::Discriminant,
        Resolver: DiagnosticMessageResolver<D>,
        C: ariadne::Cache<S::SourceId>,
    {
        let message = Resolver::message(&self.code);
        let mut builder = ariadne::Report::build(severity.into(), self.span)
            .with_config(config)
            .with_code(self.code);

        if let Some(message) = message {
            builder = builder.with_message(message);
        }

        for context in self.context_info {
            context.add_to_report_builder(&mut builder);
        }

        builder.finish().eprint(cache)
    }
}
