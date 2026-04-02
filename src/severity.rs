use core::cmp::Ordering;

/// The severity of a [`Diagnostic`](crate::Diagnostic).
///
/// This can be used to change how diagnostics are processed and reported.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticSeverity {
    /// Indicates a minor issue that may be indicative of an anti-pattern.
    Advice,

    /// Indicates an issue that, although important, is not critical enough to make the requested
    /// task fail.
    Warning,

    /// Indicates a critical issue that prevents the requested task from succeeding.
    Error,
}

impl DiagnosticSeverity {
    /// An implementation of [`Ord::cmp`] that can be used in `const` contexts.
    pub const fn ccmp(&self, rhs: &Self) -> Ordering {
        use DiagnosticSeverity::*;

        match (self, rhs) {
            (Advice, Advice) | (Warning, Warning) | (Error, Error) => Ordering::Equal,
            (Warning, Advice) | (Error, _) => Ordering::Greater,
            (Advice, _) | (Warning, _) => Ordering::Less,
        }
    }

    /// Returns the least severe of the two diagnostic severities.
    ///
    /// This is analogous to [`Ord::min`].
    pub const fn least_severe_of(a: Self, b: Self) -> Self {
        if a.less_severe_than(b) { a } else { b }
    }

    /// Returns the most severe of the two diagnostic severities.
    ///
    /// This is analogous to [`Ord::max`].
    pub const fn most_severe_of(a: Self, b: Self) -> Self {
        if a.less_severe_than(b) { b } else { a }
    }

    /// Checks if `self` is less severe than `rhs`.
    ///
    /// This is equivalent to [`self < rhs`](PartialOrd::lt).
    pub const fn less_severe_than(&self, rhs: Self) -> bool {
        self.ccmp(&rhs).is_lt()
    }

    /// Checks if `self` is at most as severe as `rhs`.
    ///
    /// This is equivalent to [`self <= rhs`](PartialOrd::ge).
    pub const fn at_most_as_severe_as(self, rhs: Self) -> bool {
        self.ccmp(&rhs).is_le()
    }

    /// Checks if `self` is more severe than `rhs`.
    ///
    /// This is equivalent to [`self > rhs`](PartialOrd::gt).
    pub const fn more_severe_than(&self, rhs: Self) -> bool {
        self.ccmp(&rhs).is_gt()
    }

    /// Checks if `self` is at least as severe as `rhs`.
    ///
    /// This is equivalent to [`self >= rhs`](PartialOrd::ge).
    pub const fn at_least_as_severe_as(self, rhs: Self) -> bool {
        self.ccmp(&rhs).is_ge()
    }
}

impl<'a> From<DiagnosticSeverity> for ariadne::ReportKind<'a> {
    fn from(severity: DiagnosticSeverity) -> Self {
        match severity {
            DiagnosticSeverity::Error => Self::Error,
            DiagnosticSeverity::Warning => Self::Warning,
            DiagnosticSeverity::Advice => Self::Advice,
        }
    }
}
