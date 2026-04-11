#![doc = include_str!("../README.md")]

extern crate proc_macro;

#[cfg(docsrs)]
pub use attribute::docs as additional_attributes;

mod attribute;
mod diagnose;
mod diagnostic_group;
mod diagnostic_info;
mod partial_diagnose;
mod utils;

use syn::{DeriveInput, parse_macro_input};

/// Implements [`Diagnose`] for a `struct` with a span and an info field.
///
/// [`Diagnose`]: https://docs.rs/maybe-fatal/latest/maybe_fatal/traits/trait.Diagnose.html
#[proc_macro_derive(Diagnose, attributes(maybe_fatal))]
pub fn diagnose_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    diagnose::parse(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Implements [`PartialDiagnose`] for a `struct` or an `enum` with `struct` variants.
///
/// [`PartialDiagnose`]: https://docs.rs/maybe-fatal/latest/maybe_fatal/traits/trait.PartialDiagnose.html
#[proc_macro_derive(PartialDiagnose, attributes(maybe_fatal))]
pub fn partial_diagnose_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    partial_diagnose::parse(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Implements [`DiagnosticGroup`] for an `enum`.
///
/// [`DiagnosticGroup`]: https://docs.rs/maybe-fatal/latest/maybe_fatal/traits/trait.DiagnosticGroup.html
#[proc_macro_derive(DiagnosticGroup, attributes(maybe_fatal))]
pub fn diagnostic_group_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    diagnostic_group::parse(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Implements [`DiagnosticGroup`] and [`PartialDiagnose`] for an `enum` with newtype variants that
/// also implement [`DiagnosticGroup`] and [`PartialDiagnose`].
///
/// [`DiagnosticGroup`]: https://docs.rs/maybe-fatal/latest/maybe_fatal/traits/trait.DiagnosticGroup.html
/// [`PartialDiagnose`]: https://docs.rs/maybe-fatal/latest/maybe_fatal/traits/trait.PartialDiagnose.html
#[proc_macro_derive(DiagnosticInfoWrapper, attributes(maybe_fatal))]
pub fn diagnostic_info_wrapper_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    diagnostic_info::parse(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
