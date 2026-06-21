//! Potentially fatal diagnostics and diagnostic handling for compilers.
//!
//! # Usage
//!
//! See the [examples] directory for some examples of how to use this crate.
//!
//! # Relationship to Other Crates
//!
//! This crate can be thought of as a high-level wrapper around the [ariadne] crate, as its core
//! functionality is driven by it. It is also kind of a spiritual successor to the [rich-err] crate
//! I wrote, but to be honest, I totally forgot that existed until I went to publish this one.
//!
//! There is also the [ariadnenum] crate, which fulfills a similar purpose to the
//! [maybe-fatal-derive] crate in this repository. The primary difference is that
//! [maybe-fatal-derive] was designed specifically for use in the context of this crate, whereas
//! [ariadnenum] is meant for a workflow that is already using [ariadne] directly.
//!
//! One other crate worth mentioning is [wurm], which was similarly made for non-fatal error
//! handling. I considered using that crate here, but it made more sense with how I had designed the
//! [`Diagnostic`] type to just make my own stuff. Functionality relating to that can be found in
//! the [`sink`] module.
//!
//! # Prerelease Status
//!
//! This is currently marked as being in beta, as I am still working on making sure the API is as
//! ergonomic and flexible as possible. I would also like to add some more detailed documentation,
//! unit tests, integration tests, and examples.
//!
//! [ariadnenum]: https://docs.rs/ariadnenum/latest/ariadnenum/
//! [examples]: https://github.com/RosieTheGhostie/maybe-fatal/tree/main/examples
//! [maybe-fatal-derive]: maybe_fatal_derive
//! [rich-err]: https://docs.rs/rich-err/latest/rich_err/
//! [wurm]: https://docs.rs/wurm/latest/wurm/

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod additional_attributes;
pub mod code;
pub mod prelude;
pub mod sink;
pub mod traits;

pub use ariadne::{self, Config, Label};

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
use traits::DiagnosticGroup;

/// A contextualized message meant to assist the user in diagnosing and resolving issues.
///
/// This is intended for use in things like parsers and compilers, but they can be used in any
/// text-based input processing.
///
/// Diagnostics are neutral by default; that is, they don't have an assigned
/// [severity](DiagnosticSeverity). To assign a severity, use the [`classify`](Self::classify)
/// method.
pub struct Diagnostic<S, D = code::DefaultDiscriminant> {
    /// The code identifying this kind of diagnostic.
    ///
    /// Diagnostics with identical codes are not necessarily equivalent. They may refer to different
    /// spans and have different contextual information.
    pub code: DiagnosticCode<D>,

    /// The span this diagnostic refers to.
    pub span: S,

    /// The diagnostic message.
    pub message: Box<str>,

    /// Any contextual information that may accompany the message.
    pub context_info: Vec<Context<S>>,
}

impl<S, D> Diagnostic<S, D> {
    /// Constructs a new [`Diagnostic`] from the provided member of a [`DiagnosticGroup`].
    pub fn new<T>(group_member: impl Borrow<T>, span: S) -> Self
    where
        T: DiagnosticGroup<D> + ?Sized,
    {
        let group_member = group_member.borrow();
        Self {
            code: group_member.diagnostic_code(),
            span,
            message: group_member.message().into_boxed_str(),
            context_info: Vec::new(),
        }
    }

    /// Labels a subspan of this diagnostic.
    ///
    /// See the documentation of [`Label`] for more details.
    pub fn label(&mut self, label: Label<S>) -> &mut Self {
        self.context_info.push(Context::Label(label));
        self
    }

    /// Sequentially labels several subspans of this diagnostic.
    ///
    /// See the documentation of [`Label`] for more details.
    pub fn labels(&mut self, labels: impl IntoIterator<Item = Label<S>>) -> &mut Self {
        self.context_info
            .extend(labels.into_iter().map(Context::Label));
        self
    }

    /// Adds a note to this diagnostic.
    pub fn note(&mut self, note: impl ToString) -> &mut Self {
        self.context_info.push(Context::new_note(note));
        self
    }

    /// Adds a help message to this diagnostic.
    pub fn help(&mut self, help: impl ToString) -> &mut Self {
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
    pub fn report_with<C>(
        self,
        severity: DiagnosticSeverity,
        config: Config,
        cache: C,
    ) -> std::io::Result<()>
    where
        S: ariadne::Span,
        D: code::Discriminant,
        C: ariadne::Cache<S::SourceId>,
    {
        use yansi::{Condition, Style};

        static BOLD: Style = Style::new().bold().whenever(Condition::STDERR_IS_TTY);
        static CODE: Style = Style::new()
            .italic()
            .dim()
            .whenever(Condition::STDERR_IS_TTY);

        let mut builder = ariadne::Report::build(ariadne::ReportKind::from(severity), self.span)
            .with_config(config)
            .with_message(format!(
                "{}{}{} {}[{}]{}",
                BOLD.prefix(),
                self.message,
                BOLD.suffix(),
                CODE.prefix(),
                self.code,
                CODE.suffix(),
            ));

        for context in self.context_info {
            context.add_to_report_builder(&mut builder);
        }

        builder.finish().eprint(cache)
    }
}

#[cfg(any(docsrs, feature = "derive"))]
mod macros {
    #[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
    pub use maybe_fatal_derive::*;
}

macro_rules! document_macro_reexports {
    [$($derive_macro:ident),* $(,)?] => {
        $(
            #[cfg(any(docsrs, feature = "derive"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
            pub use maybe_fatal_derive::$derive_macro;
        )*
    };
}

document_macro_reexports![
    Diagnose,
    DiagnosticGroup,
    DiagnosticInfoWrapper,
    PartialDiagnose,
];
