use quote::{ToTokens, quote};
use syn::{Token, parse::Parse, punctuated::Punctuated};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageMeta {
    pub fmt_string: syn::LitStr,
    pub first_comma: Option<Token![,]>,
    pub fmt_args: Punctuated<syn::Expr, Token![,]>,
}

impl Parse for MessageMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fmt_string = input.parse()?;
        Ok(if input.peek(Token![,]) {
            Self {
                fmt_string,
                first_comma: Some(input.parse()?),
                fmt_args: input.parse_terminated(syn::Expr::parse, Token![,])?,
            }
        } else {
            Self {
                fmt_string,
                first_comma: None,
                fmt_args: Punctuated::new(),
            }
        })
    }
}

impl ToTokens for MessageMeta {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            fmt_string,
            first_comma,
            fmt_args,
        } = self;
        tokens.extend(quote! {
            ::std::format!(#fmt_string #first_comma #fmt_args)
        });
    }
}
