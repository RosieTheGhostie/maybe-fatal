use core::marker::PhantomData;

use super::Sink;
use crate::{ClassifiedDiagnostic, code, traits::DiagnosticMessageResolver};

/// A [sink](Sink) that eagerly reports each diagnostic it recieves.
pub struct Report<C, Resolver> {
    /// The configuration settings that will be passed along to [`ariadne`].
    config: ariadne::Config,

    /// The source cache.
    cache: C,

    /// A marker that keeps track of which [resolver](DiagnosticMessageResolver) to use.
    _resolver: PhantomData<Resolver>,
}

impl<C, Resolver> Report<C, Resolver> {
    /// Constructs a new sink.
    pub const fn new(config: ariadne::Config, cache: C) -> Self {
        Self {
            config,
            cache,
            _resolver: PhantomData,
        }
    }
}

impl<S, D, C, Resolver> Sink<S, D> for Report<C, Resolver>
where
    S: ariadne::Span + Clone,
    D: code::Discriminant,
    C: ariadne::Cache<S::SourceId>,
    Resolver: DiagnosticMessageResolver<D>,
{
    fn add(&mut self, diagnostic: ClassifiedDiagnostic<S, D>) {
        let _ = diagnostic.report::<Resolver, _>(self.config.clone(), &mut self.cache);
    }
}
