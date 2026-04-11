use core::fmt::{self, Display, Formatter};

use maybe_fatal::{ColorPalette, prelude::*, traits::Lenient};

use crate::{
    Span,
    diagnostics::{CompilerDiagnostic, CompilerDiagnosticInfo, SemanticError},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Compiler<S> {
    sink: S,
    colors: ColorPalette,
}

impl<S> Compiler<S>
where
    S: Sink<Span>,
{
    pub fn new(sink: S, colors: ColorPalette) -> Self {
        Self { sink, colors }
    }

    pub fn compile(mut self, source_id: &'static str) -> S {
        self.error(
            (source_id, 11..58),
            SemanticError::IncompatibleTypes {
                expr_kind: "match",
                value_a_span: (source_id, 32..33),
                type_a: "Nat",
                value_b_span: (source_id, 52..55),
                type_b: "Str",
            },
        );

        self.sink
    }

    #[allow(dead_code)]
    fn error(&mut self, span: Span, info: impl Into<CompilerDiagnosticInfo>) {
        self.sink.add_error(
            CompilerDiagnostic {
                span,
                info: info.into(),
            }
            .diagnose(&self.colors),
        );
    }

    #[allow(dead_code)]
    fn warn(&mut self, span: Span, info: impl Into<CompilerDiagnosticInfo> + Lenient) {
        self.sink.add_warning(
            CompilerDiagnostic {
                span,
                info: info.into(),
            }
            .diagnose(&self.colors),
        );
    }

    #[allow(dead_code)]
    fn advice(&mut self, span: Span, info: impl Into<CompilerDiagnosticInfo> + Lenient) {
        self.sink.add_advice(
            CompilerDiagnostic {
                span,
                info: info.into(),
            }
            .diagnose(&self.colors),
        );
    }
}

pub struct Summary {
    pub n_errors: usize,
    pub n_warnings: usize,
}

impl Summary {
    pub const fn new() -> Self {
        Self {
            n_errors: 0,
            n_warnings: 0,
        }
    }

    pub const fn ok(&self) -> bool {
        self.n_errors == 0
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let verb = if self.ok() { "succeeded" } else { "failed" };
        let s_error = if self.n_errors == 1 { "" } else { "s" };
        let s_warning = if self.n_warnings == 1 { "" } else { "s" };

        write!(
            f,
            "Compiler {verb} with {} error{s_error} and {} warning{s_warning}.",
            self.n_errors, self.n_warnings,
        )
    }
}
