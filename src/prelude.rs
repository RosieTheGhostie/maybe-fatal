pub use crate::{
    Diagnostic, DiagnosticSeverity,
    code::DiagnosticCode,
    sink::{Sink, SinkExt},
    traits::{Diagnose, DiagnosticGroup, DiagnosticGroupExt, PartialDiagnose},
};

#[cfg(feature = "derive")]
pub use crate::macros::*;
