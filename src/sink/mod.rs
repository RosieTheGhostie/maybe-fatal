//! Types that can act as "sinks" for [`ClassifiedDiagnostic`]s.
//!
//! A [`Sink`] consumes each diagnostic it is given, processing it however it wants. This has
//! multiple advantages over the typical approach with [`Result`]s:
//!
//! 1. Processing does not necessarily need to stop when an error is encountered. This is especially
//!    nice for error recovery.
//! 2. Creating a diagnostic does not imply failure.
//! 3. Multiple diagnostics can result from one subroutine without resorting to ugly return types
//!    like [`Result<(T, Vec<ClassifiedDiagnostic<S, D>>), Vec<ClassifiedDiagnostic<S, D>>>`].
//! 4. The code that constructs the diagnostic doesn't have to know how it is being handled.
//! 5. Types that use [`Sink`]s can typically use them generically, so consumers can employ
//!    dependency injection.

pub mod filter;

pub use collect::Collect;
pub use filter::Filter;
pub use ignore::Ignore;
pub use report::Report;

mod collect;
mod ignore;
mod report;

use sealed::sealed;

use crate::{ClassifiedDiagnostic, Diagnostic, DiagnosticSeverity, code};

/// A trait for types that can act as sinks for [`ClassifiedDiagnostic`]s.
///
/// A sink abstracts away exactly _what_ it does with each diagnostic, so the same high-level code
/// can be used to do a variety of different things.
pub trait Sink<S, D = code::DefaultDiscriminant> {
    /// Adds a diagnostic to the sink.
    ///
    /// The trait gives no guarantees as to what happens when this method is called. It may or may
    /// not depend on internal/external state, and it may or may not actually _do_ anything with the
    /// diagnostic.
    fn add(&mut self, diagnostic: ClassifiedDiagnostic<S, D>);
}

/// Provides a number of extension methods for [sink](Sink)s.
///
/// # Note to Consumers
///
/// This trait is **sealed**; it cannot be implemented outside the [`maybe_fatal`](crate) crate. If
/// you want your type to implement this trait, implement [`Sink`].
///
/// The reason for this, by the way, is to provide more guarantees about what each of the methods
/// do. By sealing the trait, this crate can guarantee that these methods do what they say they do.
#[sealed]
pub trait SinkExt<S, D = code::DefaultDiscriminant>: Sink<S, D> {
    /// Adds the given diagnostic to the sink as an error.
    fn add_error(&mut self, diagnostic: Diagnostic<S, D>) {
        self.add(diagnostic.classify(DiagnosticSeverity::Error));
    }

    /// Adds the given diagnostic to the sink as a warning.
    fn add_warning(&mut self, diagnostic: Diagnostic<S, D>) {
        self.add(diagnostic.classify(DiagnosticSeverity::Warning));
    }

    /// Adds the given diagnostic to the sink as advice.
    fn add_advice(&mut self, diagnostic: Diagnostic<S, D>) {
        self.add(diagnostic.classify(DiagnosticSeverity::Advice));
    }

    /// Filters diagnostics provided to this sink with the given predicate.
    ///
    /// A diagnostic will be passed along to the inner sink (i.e., `self`) if and only if the
    /// predicate returns `true`.
    ///
    /// A callback can also be added whenever a diagnostic fulfills the predicate via
    /// [`Filter::with_add_callback`].
    fn filter<C>(self, context: C, filter: filter::FilterFn<C, S, D>) -> Filter<Self, C, S, D>
    where
        Self: Sized,
    {
        Filter::new(self, context, filter)
    }

    /// Ignores all diagnostics provided to this sink.
    ///
    /// Any existing information stored in this sink is lost.
    fn ignore(self) -> Ignore<S>
    where
        Self: Sized,
    {
        Ignore::new()
    }
}

#[sealed]
impl<T, S, D> SinkExt<S, D> for T where T: Sink<S, D> {}
