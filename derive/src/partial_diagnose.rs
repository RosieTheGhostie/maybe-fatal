use core::iter::zip;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::{attribute, utils};

pub fn parse(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let syn::DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = input;

    let helper_info = attribute::item::MaybeFatal::parse_from_attrs(&attrs)?;

    let Some(span_type) = helper_info.span_type else {
        return Err(syn::Error::new_spanned(
            ident,
            "missing `span_type` meta attribute",
        ));
    };

    let discriminant_type = helper_info
        .group
        .and_then(|group| group.discriminant_type)
        .unwrap_or_else(|| parse_quote!(::maybe_fatal::code::DefaultDiscriminant));

    let method_body = utils::process_data(
        data,
        |data| parse_struct(quote! { self. }, &data.fields),
        parse_enum,
        utils::deny_data!("cannot derive `PartialDiagnose` for `union`s"),
    )?;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let where_clause: syn::WhereClause = match where_clause {
        Some(syn::WhereClause {
            where_token,
            predicates,
        }) => {
            let predicates = predicates.iter();
            syn::parse_quote! { #where_token #(#predicates),* #span_type: ::ariadne::Span }
        }
        None => syn::parse_quote! { where #span_type: ::ariadne::Span },
    };

    Ok(quote! {
        impl #impl_generics ::maybe_fatal::traits::PartialDiagnose<#span_type, #discriminant_type>
            for #ident #ty_generics
        #where_clause
        {
            fn partial_diagnose(
                self,
                mut diagnostic: ::maybe_fatal::Diagnostic<#span_type, #discriminant_type>,
                colors: &::maybe_fatal::ColorPalette,
            ) -> ::maybe_fatal::Diagnostic<#span_type, #discriminant_type> {
                #method_body

                diagnostic
            }
        }
    })
}

fn parse_struct(self_dot: TokenStream, fields: &syn::Fields) -> syn::Result<TokenStream> {
    let mut body = TokenStream::new();
    for (field, member) in zip(fields.iter(), fields.members()) {
        let field_info = attribute::field::MaybeFatal::parse_from_attrs(&field.attrs)?;
        if let Some(label_meta) = field_info.label {
            body.extend(quote! {
                diagnostic.label(::ariadne::Label::new(#self_dot #member) #label_meta);
            });
        }
    }

    Ok(body)
}

fn parse_enum(data: syn::DataEnum) -> syn::Result<TokenStream> {
    let mut match_body = TokenStream::new();
    for variant in data.variants {
        let ident = variant.ident;
        let arm_body = parse_struct(TokenStream::new(), &variant.fields)?;
        let members = variant.fields.members();
        match_body.extend(quote! { Self::#ident { #(#members),* } => { #arm_body } })
    }

    Ok(quote! { match self { #match_body } })
}
