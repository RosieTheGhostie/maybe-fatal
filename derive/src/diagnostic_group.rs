use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    attribute::{self, Repr, item::GroupMeta},
    utils,
};

pub fn parse(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let repr = Repr::parse_from_attrs(&input.attrs)?;
    if repr.is_rust() || repr.is_transparent() {
        return Err(syn::Error::new(
            repr.layout_span_or_call_site(),
            "must use primitive or C layout",
        ));
    }

    let helper_info = attribute::item::MaybeFatal::parse_from_attrs(&input.attrs)?;

    let Some(GroupMeta {
        prefix: Some(prefix),
        discriminant_type,
    }) = helper_info.group
    else {
        return Err(syn::Error::new_spanned(
            input.ident,
            "missing `prefix` meta attribute of `group`",
        ));
    };

    let discriminant_type = discriminant_type
        .unwrap_or_else(|| syn::parse_quote!(::maybe_fatal::code::DefaultDiscriminant));

    let syn::DataEnum { variants, .. } = utils::try_into_data_enum(
        input.data,
        || "cannot derive `DiagnosticGroup` for `struct`s",
        || "cannot derive `DiagnosticGroup` for `union`s",
    )?;

    let mut message_match_arms = TokenStream::new();
    for syn::Variant {
        attrs: variant_attrs,
        ident: variant_name,
        fields,
        ..
    } in &variants
    {
        let attribute::variant::MaybeFatal {
            message: Some(message),
            ..
        } = attribute::variant::MaybeFatal::parse_from_attrs(variant_attrs)?
        else {
            return Err(syn::Error::new_spanned(
                variant_name,
                "missing `message` meta attribute",
            ));
        };

        let members = fields.members();
        message_match_arms.extend(quote! {
            Self::#variant_name { #(#members),* } => #message,
        });
    }

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::maybe_fatal::traits::DiagnosticGroup<#discriminant_type>
            for #name #ty_generics
        #where_clause
        {
            fn message(&self) -> ::std::string::String {
                #![allow(unused)]
                match self { #message_match_arms }
            }

            fn diagnostic_code(&self) -> ::maybe_fatal::code::DiagnosticCode<#discriminant_type> {
                unsafe {
                    ::maybe_fatal::code::DiagnosticCode::<#discriminant_type>::new_unchecked(
                        [#(#prefix),*],
                        *::core::ptr::from_ref(self).cast::<#discriminant_type>(),
                    )
                }
            }
        }
    })
}
