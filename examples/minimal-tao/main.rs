use ariadne::Fmt;
use maybe_fatal::{ColorPalette, prelude::*};

const SOURCE_ID: &str = "sample.tao";
const SOURCE: &str = include_str!("../sample.tao");

type Span = (&'static str, core::ops::Range<usize>);

#[derive(Clone, Debug, DiagnosticGroup, PartialDiagnose)]
#[maybe_fatal(span_type = Span, group(prefix = "SEM"))]
#[repr(u8)]
enum SemanticError {
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

        type_a: String,

        #[maybe_fatal(label(
            message("This is of type {}", type_b.fg(colors.random[1])),
            color = colors.random[1],
        ))]
        value_b_span: Span,

        type_b: String,
    } = 3,
}

fn main() -> std::io::Result<()> {
    SemanticError::IncompatibleTypes {
        expr_kind: "match",
        value_a_span: (SOURCE_ID, 32..33),
        type_a: "Nat".into(),
        value_b_span: (SOURCE_ID, 52..55),
        type_b: "Str".into(),
    }
    .into_diagnostic((SOURCE_ID, 11..58), &ColorPalette::new())
    .classify(DiagnosticSeverity::Error)
    .report(
        ariadne::Config::new(),
        ariadne::sources([(SOURCE_ID, SOURCE)]),
    )
}
