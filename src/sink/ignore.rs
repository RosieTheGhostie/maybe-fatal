use core::marker::PhantomData;

use super::Sink;
use crate::ClassifiedDiagnostic;

/// A [sink](Sink) that ignores all diagnostics it is given.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Ignore<S>(PhantomData<S>);

impl<S> Ignore<S> {
    /// Constructs a new sink.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<S, D> Sink<S, D> for Ignore<S> {
    fn add(&mut self, _: ClassifiedDiagnostic<S, D>) {}
}
