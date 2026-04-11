use ariadne::Fmt;
use maybe_fatal::prelude::*;

use crate::Span;

#[derive(Clone, Debug, Diagnose)]
#[maybe_fatal(span_type = Span)]
pub struct CompilerDiagnostic {
    #[maybe_fatal(span)]
    pub span: Span,

    #[maybe_fatal(info)]
    pub info: CompilerDiagnosticInfo,
}

#[derive(Clone, Debug, DiagnosticInfoWrapper)]
#[maybe_fatal(span_type = Span)]
pub enum CompilerDiagnosticInfo {
    Semantic(SemanticError),
}

#[derive(Clone, Debug, DiagnosticGroup, PartialDiagnose)]
#[maybe_fatal(span_type = Span, group(prefix = "SEM"))]
#[repr(u8)]
pub enum SemanticError {
    #[maybe_fatal(
        message("Incompatible types"),
        label(
            message("The values are outputs of this {} expression", expr_kind.fg(colors.special)),
            color = colors.special,
            order = i32::MAX,
        ),
        note(
            "Outputs of {} expressions must coerce to the same type",
            expr_kind.fg(colors.special),
        ),
    )]
    IncompatibleTypes {
        expr_kind: &'static str,

        #[maybe_fatal(label(
            message("This is of type {}", type_a.fg(colors.random[0])),
            color = colors.random[0],
        ))]
        value_a_span: Span,

        type_a: &'static str,

        #[maybe_fatal(label(
            message("This is of type {}", type_b.fg(colors.random[1])),
            color = colors.random[1],
        ))]
        value_b_span: Span,

        type_b: &'static str,
    } = 3,
}
