use super::Sink;
use crate::{ClassifiedDiagnostic, ColorPalette};

/// A trait for types that have a [`ColorPalette`] available to them.
pub trait HasColorPalette {
    /// Immutably borrows a [`ColorPalette`].
    fn colors(&self) -> &ColorPalette;
}

impl HasColorPalette for ColorPalette {
    fn colors(&self) -> &ColorPalette {
        self
    }
}

/// A [sink](Sink) adapter that converts an arbitrary sink into one that implements
/// [`HasColorPalette`].
///
/// The primary benefit of this is the ability to use the `diagnose_as_*` extension methods.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Colored<S> {
    pub inner: S,
    colors: ColorPalette,
}

impl<S> Colored<S> {
    pub const fn new(sink: S, colors: ColorPalette) -> Self {
        Self {
            inner: sink,
            colors,
        }
    }

    pub const fn new_with_default(sink: S) -> Self {
        Self::new(sink, ColorPalette::new())
    }
}

impl<S> HasColorPalette for Colored<S> {
    fn colors(&self) -> &ColorPalette {
        &self.colors
    }
}

impl<I, S, D> Sink<S, D> for Colored<I>
where
    I: Sink<S, D>,
{
    fn add(&mut self, diagnostic: ClassifiedDiagnostic<S, D>) {
        self.inner.add(diagnostic)
    }
}
