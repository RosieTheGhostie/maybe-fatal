use syn::{Attribute, meta::ParseNestedMeta};

use crate::attribute::field::LabelMeta;

use super::message::MessageMeta;

#[derive(Clone, Default)]
pub struct MaybeFatal {
    pub message: Option<MessageMeta>,
    pub label: Option<LabelMeta>,
    pub notes: Vec<MessageMeta>,
    pub help_messages: Vec<MessageMeta>,
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
        if meta.path.is_ident("message") {
            self.parse_message(meta)
        } else if meta.path.is_ident("label") {
            self.parse_label(meta)
        } else if meta.path.is_ident("note") {
            self.parse_note(meta)
        } else if meta.path.is_ident("help") {
            self.parse_help(meta)
        } else {
            Err(meta.error("unrecognized meta attribute"))
        }
    }

    fn parse_message(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.message.is_some() {
            return Err(meta.error("repeated `message` meta attribute"));
        }

        let content;
        let _ = syn::parenthesized!(content in meta.input);
        self.message = Some(content.parse()?);

        Ok(())
    }

    fn parse_label(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        if self.label.is_some() {
            return Err(meta.error("repeated `label` meta attribute"));
        }

        self.label = Some(LabelMeta::parse_from_nested_meta(meta)?);

        Ok(())
    }

    fn parse_note(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        let content;
        let _ = syn::parenthesized!(content in meta.input);
        self.notes.push(content.parse()?);

        Ok(())
    }

    fn parse_help(&mut self, meta: ParseNestedMeta<'_>) -> syn::Result<()> {
        let content;
        let _ = syn::parenthesized!(content in meta.input);
        self.help_messages.push(content.parse()?);

        Ok(())
    }
}
