pub use self::case_style::{CaseStyleHelpers, snakify};
pub use self::type_props::HasTypeProperties;
pub use self::variant_props::HasStrumVariantProperties;

pub mod case_style;
mod metadata;
pub mod type_props;
pub mod variant_props;

use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;

pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

pub fn non_new_type_variant_error(additional_info: &str) -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        format!(
            "This macro only supports enums of strictly new type variants, but {additional_info}"
        ),
    )
}

pub fn no_associated_deref_type_specified() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "expected a deref target specified via attribute, e.g. #[strum_deref(T)]",
    )
}

pub fn strum_discriminants_passthrough_error(span: &impl Spanned) -> syn::Error {
    syn::Error::new(
        span.span(),
        "expected a pass-through attribute, e.g. #[strum_discriminants(serde(rename = \"var0\"))]",
    )
}

pub fn occurrence_error<T: ToTokens>(fst: T, snd: T, attr: &str) -> syn::Error {
    let mut e = syn::Error::new_spanned(
        snd,
        format!("Found multiple occurrences of strum({})", attr),
    );
    e.combine(syn::Error::new_spanned(fst, "first one here"));
    e
}
