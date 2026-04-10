use quote::{ToTokens, quote};
use syn::{Attribute, Token, meta::ParseNestedMeta};

#[derive(Clone, Default)]
pub struct MaybeFatal {
    pub kind: Option<FieldKind>,
    pub label: Option<LabelMeta>,
}

impl MaybeFatal {
    pub fn parse_from_attrs<'a>(
        attrs: impl IntoIterator<Item = &'a Attribute>,
    ) -> syn::Result<Self> {
        let mut this = Self::default();
        for attr in attrs {
            this.try_extend_with(attr)?;
        }

        Ok(this)
    }

    pub fn try_extend_with(&mut self, attr: &Attribute) -> syn::Result<()> {
        if attr.path().is_ident("maybe_fatal") {
            attr.parse_nested_meta(|meta| self.parse_nested_subattribute(meta))
        } else {
            Ok(())
        }
    }

    fn parse_nested_subattribute(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if meta.path.is_ident("info") {
            self.parse_kind_info(meta)
        } else if meta.path.is_ident("span") {
            self.parse_kind_span(meta)
        } else if meta.path.is_ident("label") {
            self.parse_label(meta)
        } else {
            Err(meta.error("unrecognized meta attribute"))
        }
    }

    fn parse_kind_info(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.kind.is_some() {
            return Err(meta.error("multiple field kind specifiers"));
        } else if !meta.input.is_empty() {
            return Err(meta.error("meta attribute `info` with non-empty input"));
        }

        self.kind = Some(FieldKind::Info);

        Ok(())
    }

    fn parse_kind_span(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.kind.is_some() {
            return Err(meta.error("multiple field kind specifiers"));
        } else if !meta.input.is_empty() {
            return Err(meta.error("meta attribute `span` with non-empty input"));
        }

        self.kind = Some(FieldKind::Span);

        Ok(())
    }

    fn parse_label(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.label.is_some() {
            return Err(meta.error("repeated `label` meta attribute"));
        }

        self.label = Some(LabelMeta::parse_from_nested_meta(meta)?);

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FieldKind {
    Info,
    Span,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LabelMeta {
    pub message: Option<syn::Expr>,
    pub color: Option<syn::Expr>,
    pub order: Option<syn::Expr>,
}

impl LabelMeta {
    pub fn parse_from_nested_meta(meta: ParseNestedMeta<'_>) -> syn::Result<Self> {
        let mut this = Self::default();
        meta.parse_nested_meta(|meta| this.parse_nested_subattribute(meta))?;

        Ok(this)
    }

    fn parse_nested_subattribute(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if meta.path.is_ident("message") {
            self.parse_message(meta)
        } else if meta.path.is_ident("color") {
            self.parse_color(meta)
        } else if meta.path.is_ident("order") {
            self.parse_order(meta)
        } else {
            Err(meta.error("unrecognized meta attribute"))
        }
    }

    fn parse_message(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.message.is_some() {
            return Err(meta.error("repeated `message` meta attribute"));
        }

        let _: Token![=] = meta.input.parse()?;
        self.message = Some(meta.input.parse()?);

        Ok(())
    }

    fn parse_color(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.color.is_some() {
            return Err(meta.error("repeated `color` meta attribute"));
        }

        let _: Token![=] = meta.input.parse()?;
        self.color = Some(meta.input.parse()?);

        Ok(())
    }

    fn parse_order(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.order.is_some() {
            return Err(meta.error("repeated `order` meta attribute"));
        }

        let _: Token![=] = meta.input.parse()?;
        self.order = Some(meta.input.parse()?);

        Ok(())
    }
}

impl ToTokens for LabelMeta {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(message) = &self.message {
            tokens.extend(quote! { .with_message(#message) });
        }

        if let Some(color) = &self.color {
            tokens.extend(quote! { .with_color(#color) });
        }

        if let Some(order) = &self.order {
            tokens.extend(quote! { .with_order(#order) });
        }
    }
}
