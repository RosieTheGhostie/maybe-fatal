use std::fmt::Display;

macro_rules! deny_data {
    ($error:expr) => {
        |data| <_ as $crate::utils::_SynDataVariant>::deny(&data, $error)
    };
}
pub(crate) use deny_data;

pub fn process_data<T>(
    data: syn::Data,
    on_struct: impl FnOnce(syn::DataStruct) -> syn::Result<T>,
    on_enum: impl FnOnce(syn::DataEnum) -> syn::Result<T>,
    on_union: impl FnOnce(syn::DataUnion) -> syn::Result<T>,
) -> syn::Result<T> {
    match data {
        syn::Data::Struct(data_struct) => on_struct(data_struct),
        syn::Data::Enum(data_enum) => on_enum(data_enum),
        syn::Data::Union(data_union) => on_union(data_union),
    }
}

pub fn try_into_data_struct<E, U>(
    data: syn::Data,
    enum_error: impl FnOnce() -> E,
    union_error: impl FnOnce() -> U,
) -> syn::Result<syn::DataStruct>
where
    E: Display,
    U: Display,
{
    process_data(
        data,
        Ok,
        deny_data!(enum_error()),
        deny_data!(union_error()),
    )
}

pub fn try_into_data_enum<S, U>(
    data: syn::Data,
    struct_error: impl FnOnce() -> S,
    union_error: impl FnOnce() -> U,
) -> syn::Result<syn::DataEnum>
where
    S: Display,
    U: Display,
{
    process_data(
        data,
        deny_data!(struct_error()),
        Ok,
        deny_data!(union_error()),
    )
}

pub trait _SynDataVariant {
    fn kw_token_span(&self) -> proc_macro2::Span;

    fn deny<T>(&self, error: impl Display) -> syn::Result<T> {
        Err(syn::Error::new(self.kw_token_span(), error))
    }
}

impl _SynDataVariant for syn::DataStruct {
    fn kw_token_span(&self) -> proc_macro2::Span {
        self.struct_token.span
    }
}

impl _SynDataVariant for syn::DataEnum {
    fn kw_token_span(&self) -> proc_macro2::Span {
        self.enum_token.span
    }
}

impl _SynDataVariant for syn::DataUnion {
    fn kw_token_span(&self) -> proc_macro2::Span {
        self.union_token.span
    }
}
