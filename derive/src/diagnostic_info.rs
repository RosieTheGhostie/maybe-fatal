use proc_macro2::TokenStream;
use quote::quote;

use crate::{attribute, utils};

pub fn parse(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let helper_info = attribute::item::MaybeFatal::parse_from_attrs(&input.attrs)?;

    let Some(span_type) = helper_info.span_type else {
        return Err(syn::Error::new_spanned(
            input.ident,
            "missing `span_type` meta attribute",
        ));
    };

    let discriminant_type = helper_info
        .group
        .and_then(|group| group.discriminant_type)
        .unwrap_or_else(|| syn::parse_quote!(::maybe_fatal::code::DefaultDiscriminant));

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let extended_where_clause: syn::WhereClause = match where_clause {
        Some(syn::WhereClause {
            where_token,
            predicates,
        }) => {
            let predicates = predicates.iter();
            syn::parse_quote! { #where_token #(#predicates),* #span_type: ::ariadne::Span }
        }
        None => syn::parse_quote! { where #span_type: ::ariadne::Span },
    };

    let syn::DataEnum { variants, .. } = utils::try_into_data_enum(
        input.data,
        || "cannot use derive macro `DiagnosticInfoWrapper` on `struct`s",
        || "cannot use derive macro `DiagnosticInfoWrapper` on `union`s",
    )?;

    let mut diagnostic_group_message_match_arms = TokenStream::new();
    let mut diagnostic_group_code_match_arms = TokenStream::new();
    let mut partial_diagnose_match_arms = TokenStream::new();
    let mut from_impls = TokenStream::new();
    for syn::Variant { ident, fields, .. } in variants {
        let mut fields_iter = fields.iter();
        let Some(syn::Field { ty: field_ty, .. }) = fields_iter.next() else {
            return Err(syn::Error::new(ident.span(), "expected non-unit variant"));
        };

        diagnostic_group_message_match_arms.extend(quote! {
            Self::#ident(info) => <_ as ::maybe_fatal::traits::DiagnosticGroup<#discriminant_type>>::message(info),
        });
        diagnostic_group_code_match_arms.extend(quote! {
            Self::#ident(info) => <_ as ::maybe_fatal::traits::DiagnosticGroup<#discriminant_type>>::diagnostic_code(info),
        });
        partial_diagnose_match_arms.extend(quote! {
            Self::#ident(info) => <_ as ::maybe_fatal::traits::PartialDiagnose<_, #discriminant_type>>::partial_diagnose(info, diagnostic, colors),
        });
        from_impls.extend(quote! {
            impl #impl_generics ::core::convert::From<#field_ty> for #name #ty_generics
            #where_clause
            {
                fn from(info: #field_ty) -> Self {
                    Self::#ident(info)
                }
            }
        });
    }

    Ok(quote! {
        impl #impl_generics ::maybe_fatal::traits::DiagnosticGroup<#discriminant_type>
            for #name #ty_generics
        #where_clause
        {
            fn message(
                &self,
            ) -> ::std::boxed::Box<dyn ::core::ops::FnOnce() -> ::std::string::String> {
                match self { #diagnostic_group_message_match_arms }
            }

            fn diagnostic_code(&self) -> ::maybe_fatal::code::DiagnosticCode<#discriminant_type> {
                match self { #diagnostic_group_code_match_arms }
            }
        }

        impl #impl_generics ::maybe_fatal::traits::PartialDiagnose<#span_type, #discriminant_type>
            for #name #ty_generics
        #extended_where_clause
        {
            fn partial_diagnose(
                self,
                diagnostic: ::maybe_fatal::Diagnostic<#span_type, #discriminant_type>,
                colors: &::maybe_fatal::ColorPalette,
            ) -> ::maybe_fatal::Diagnostic<#span_type, #discriminant_type> {
                #![allow(unused)]
                match self { #partial_diagnose_match_arms }
            }
        }

        #from_impls
    })
}
