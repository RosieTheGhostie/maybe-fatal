//! The [`Collect`] sink.

use super::Sink;
use crate::{ClassifiedDiagnostic, code};

/// A [sink](Sink) that collects all the diagnostics it receives.
///
/// The diagnostics can be inspected through [`Self::diagnostics`] and retrieved through
/// [`Self::take_diagnostics`].
#[derive(Default)]
pub struct Collect<S, D = code::DefaultDiscriminant> {
    diagnostics: Vec<ClassifiedDiagnostic<S, D>>,
}

impl<S, D> Collect<S, D> {
    /// Constructs an empty sink.
    pub const fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Immutably borrows all the diagnostics that have been collected thus far.
    pub const fn diagnostics(&self) -> &[ClassifiedDiagnostic<S, D>] {
        &self.diagnostics.as_slice()
    }

    /// Yields ownership of all the diagnostics that have been collected.
    pub fn take_diagnostics(self) -> Box<[ClassifiedDiagnostic<S, D>]> {
        self.diagnostics.into_boxed_slice()
    }
}

impl<S, D> Sink<S, D> for Collect<S, D> {
    fn add(&mut self, diagnostic: ClassifiedDiagnostic<S, D>) {
        self.diagnostics.push(diagnostic);
    }
}
