use syn::{Attribute, LitStr, Token, meta::ParseNestedMeta};

#[derive(Clone, Default)]
pub struct MaybeFatal {
    pub span_type: Option<syn::Type>,
    pub group: Option<GroupMeta>,
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
        if meta.path.is_ident("span_type") {
            self.parse_span_type(meta)
        } else if meta.path.is_ident("group") {
            self.parse_group(meta)
        } else {
            Err(meta.error("unrecognized meta attribute"))
        }
    }

    fn parse_span_type(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.span_type.is_some() {
            return Err(meta.error("repeated `span_type` meta attribute"));
        }

        let _: Token![=] = meta.input.parse()?;
        self.span_type = Some(meta.input.parse()?);

        Ok(())
    }

    fn parse_group(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.group.is_some() {
            return Err(meta.error("repeated `group` meta attribute"));
        }

        self.group = Some(GroupMeta::parse_from_nested_meta(meta)?);

        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct GroupMeta {
    pub prefix: Option<[u8; 3]>,
    pub discriminant_type: Option<syn::Type>,
}

impl GroupMeta {
    pub fn parse_from_nested_meta(meta: ParseNestedMeta<'_>) -> syn::Result<Self> {
        let mut this = Self::default();
        meta.parse_nested_meta(|meta| this.parse_nested_subattribute(meta))?;

        Ok(this)
    }

    fn parse_nested_subattribute(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if meta.path.is_ident("prefix") {
            self.parse_prefix(meta)
        } else if meta.path.is_ident("discriminant_type") {
            self.parse_discriminant_type(meta)
        } else {
            Err(meta.error("unrecognized meta attribute"))
        }
    }

    fn parse_prefix(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.prefix.is_some() {
            return Err(meta.error("repeated `prefix` meta attribute"));
        }

        let _: Token![=] = meta.input.parse()?;
        let lit_prefix: LitStr = meta.input.parse()?;
        self.prefix = Some(lit_prefix.value().into_bytes().try_into().map_err(|_| {
            syn::Error::new(lit_prefix.span(), "prefix is incorrect length (expected 3)")
        })?);

        Ok(())
    }

    fn parse_discriminant_type(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.discriminant_type.is_some() {
            return Err(meta.error("repeated `discriminant_type` meta attribute"));
        }

        let _: Token![=] = meta.input.parse()?;
        self.discriminant_type = Some(meta.input.parse()?);

        Ok(())
    }
}
