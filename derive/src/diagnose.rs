use core::iter::zip;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    attribute::{self, field::FieldKind},
    utils,
};

pub fn parse(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let helper_info = attribute::item::MaybeFatal::parse_from_attrs(&input.attrs)?;

    let Some(span_type) = helper_info.span_type else {
        return Err(syn::Error::new_spanned(
            input.ident,
            "missing `span_type` meta attribute",
        ));
    };

    let syn::DataStruct { fields, .. } = utils::try_into_data_struct(
        input.data,
        || "cannot derive `Diagnose` for `enum`s",
        || "cannot derive `Diagnose` for `union`s",
    )?;

    let mut info_member = None::<syn::Member>;
    let mut span_member = None::<syn::Member>;

    for (field, member) in zip(fields.iter(), fields.members()) {
        let field_info = attribute::field::MaybeFatal::parse_from_attrs(&field.attrs)?;
        match field_info.kind {
            Some(FieldKind::Info) if info_member.is_none() => info_member = Some(member),
            Some(FieldKind::Info) => {
                return Err(syn::Error::new_spanned(
                    field,
                    "multiple fields marked with `info` meta attribute",
                ));
            }
            Some(FieldKind::Span) if span_member.is_none() => span_member = Some(member),
            Some(FieldKind::Span) => {
                return Err(syn::Error::new_spanned(
                    field,
                    "multiple fields marked with `span` meta attribute",
                ));
            }
            None => {}
        }
    }

    let Some(info_member) = info_member else {
        return Err(syn::Error::new(
            input.ident.span(),
            "missing field marked with `info` meta attribute",
        ));
    };

    let Some(span_member) = span_member else {
        return Err(syn::Error::new(
            input.ident.span(),
            "missing field marked with `span` meta attribute",
        ));
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let where_clause = utils::adjust_where_clause(where_clause, &span_type);

    Ok(quote! {
        impl #impl_generics ::maybe_fatal::traits::Diagnose<#span_type> for #name #ty_generics
        #where_clause
        {
            fn diagnose(
                self,
                colors: &::maybe_fatal::ColorPalette,
            ) -> ::maybe_fatal::Diagnostic<#span_type> {
                self.#info_member.into_diagnostic(self.#span_member, colors)
            }
        }
    })
}
