use proc_macro2::Span;
use syn::{Attribute, meta::ParseNestedMeta};

#[derive(Clone, Default)]
pub struct Repr {
    pub layout: Option<syn::Ident>,
    pub packed: Option<syn::LitInt>,
    pub align: Option<syn::LitInt>,
}

impl Repr {
    pub fn layout_span_or_call_site(&self) -> Span {
        match &self.layout {
            Some(layout) => layout.span(),
            None => Span::call_site(),
        }
    }

    pub fn is_rust(&self) -> bool {
        self.layout.as_ref().is_none_or(|layout| layout == "Rust")
    }

    pub fn is_transparent(&self) -> bool {
        self.layout
            .as_ref()
            .is_some_and(|layout| layout == "transparent")
    }

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
        if attr.path().is_ident("repr") {
            attr.parse_nested_meta(|meta| self.parse_nested_subattribute(meta))
        } else {
            Ok(())
        }
    }

    fn parse_nested_subattribute(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if meta.path.is_ident("packed") {
            self.parse_packed(meta)
        } else if meta.path.is_ident("align") {
            self.parse_align(meta)
        } else {
            self.parse_layout(meta)
        }
    }

    fn parse_layout(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.layout.is_some() {
            return Err(meta.error("multiple layouts"));
        }

        self.layout = Some(meta.path.require_ident()?.clone());
        if meta.input.is_empty() {
            Ok(())
        } else {
            Err(meta.error("expected layout only"))
        }
    }

    fn parse_packed(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.packed.is_some() {
            return Err(meta.error("repeated `packed` meta attribute"));
        }

        if meta.input.is_empty() {
            self.packed = Some(syn::parse_quote! { 1 });
            return Ok(());
        }

        let content;
        syn::parenthesized!(content in meta.input);
        self.packed = Some(content.parse()?);

        if content.is_empty() {
            Ok(())
        } else {
            Err(meta.error("expected end of `packed` meta attribute"))
        }
    }

    fn parse_align(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.align.is_some() {
            return Err(meta.error("repeated `align` meta attribute"));
        }

        let content;
        syn::parenthesized!(content in meta.input);
        self.align = Some(content.parse()?);

        if content.is_empty() {
            Ok(())
        } else {
            Err(meta.error("expected end of `align` meta attribute"))
        }
    }
}
