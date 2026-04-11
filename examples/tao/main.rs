#![doc = include_str!("README.md")]

mod compiler;
mod diagnostics;

use maybe_fatal::{ColorPalette, prelude::*, sink};

use compiler::Compiler;

type Span = (&'static str, core::ops::Range<usize>);

const SOURCE_ID: &str = "sample.tao";
const SOURCE: &str = include_str!("../sample.tao");

fn main() {
    let sink = sink::Report::from_sources([(SOURCE_ID, SOURCE)])
        .filter(compiler::Summary::new(), |_, _| true)
        .with_add_callback(|summary, diagnostic| {
            if diagnostic.severity == DiagnosticSeverity::Error {
                summary.n_errors += 1;
            } else if diagnostic.severity == DiagnosticSeverity::Warning {
                summary.n_warnings += 1;
            }
        });

    let sink::Filter {
        context: summary, ..
    } = Compiler::new(sink, ColorPalette::new()).compile(SOURCE_ID);
    println!("{summary}");
}
