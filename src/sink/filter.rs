use super::Sink;
use crate::{ClassifiedDiagnostic, DiagnosticSeverity, code};

/// A function that can be used to filter diagnostics.
pub type FilterFn<C, S, D = code::DefaultDiscriminant> =
    fn(&C, &ClassifiedDiagnostic<S, D>) -> bool;

/// A callback function that is called whenever a diagnostic is added to the inner sink.
pub type AddCallback<C, S, D = code::DefaultDiscriminant> = fn(&mut C, &ClassifiedDiagnostic<S, D>);

/// A [filter function](FilterFn) that returns `true` if and only if `diagnostic` has a severity
/// that is at least as severe as [`Warning`](DiagnosticSeverity::Warning).
pub const fn at_least_warning<C, S, D>(
    _context: &C,
    diagnostic: &ClassifiedDiagnostic<S, D>,
) -> bool {
    diagnostic
        .severity
        .at_least_as_severe_as(&DiagnosticSeverity::Warning)
}

/// A [filter function](FilterFn) that returns `true` if and only if `diagnostic` has a severity
/// that is at least as severe as [`Error`](DiagnosticSeverity::Error).
pub const fn at_least_error<C, S, D>(
    _context: &C,
    diagnostic: &ClassifiedDiagnostic<S, D>,
) -> bool {
    diagnostic
        .severity
        .at_least_as_severe_as(&DiagnosticSeverity::Error)
}

/// A [sink](Sink) adapter that only accepts diagnostics satisfying some predicate.
#[derive(Clone, Debug, Hash)]
pub struct Filter<I, C, S, D = code::DefaultDiscriminant> {
    /// The inner sink.
    inner: I,

    /// Contextual information that may be used to influence the predicate.
    pub context: C,

    /// The predicate determining what gets passed along to the [inner sink](Self::inner).
    predicate: FilterFn<C, S, D>,

    /// A callback that is invoked whenever a diagnostic satisfies the [predicate](Self::predicate).
    add_callback: AddCallback<C, S, D>,
}

impl<I, C, S, D> Filter<I, C, S, D> {
    /// Constructs a new [`Filter`].
    pub const fn new(sink: I, context: C, filter: FilterFn<C, S, D>) -> Self {
        Self {
            inner: sink,
            context,
            predicate: filter,
            add_callback: |_, _| {},
        }
    }

    /// Sets the callback that will be invoked each time a diagnostic satisfies the predicate.
    ///
    /// Unlike [`with_add_callback`], this borrows `self`, so it cannot be used as the last builder
    /// method in a chain; however, it can be called by itself without reassigning `self`'s owner.
    ///
    /// # See Also
    ///
    /// - [`with_add_callback`]
    ///
    /// [`with_add_callback`]: Self::with_add_callback
    pub const fn set_add_callback(&mut self, callback: AddCallback<C, S, D>) -> &mut Self {
        self.add_callback = callback;
        self
    }

    /// Sets the callback that will be invoked each time a diagnostic satisfies the predicate.
    ///
    /// Unlike [`set_add_callback`], this consumes `self`, so it can be used as the last builder
    /// method in a chain; however, it cannot be called by itself without reassigning `self`'s
    /// owner.
    ///
    /// # See Also
    ///
    /// - [`set_add_callback`]
    ///
    /// [`set_add_callback`]: Self::set_add_callback
    pub const fn with_add_callback(mut self, callback: AddCallback<C, S, D>) -> Self {
        self.set_add_callback(callback);
        self
    }
}

impl<I, C, S, D> Sink<S, D> for Filter<I, C, S, D>
where
    I: Sink<S, D>,
{
    fn add(&mut self, diagnostic: ClassifiedDiagnostic<S, D>) {
        if (self.predicate)(&self.context, &diagnostic) {
            (self.add_callback)(&mut self.context, &diagnostic);
            self.inner.add(diagnostic);
        }
    }
}
