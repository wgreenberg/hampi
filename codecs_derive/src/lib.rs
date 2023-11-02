//! Derive Macros for Rust Structures generated by ASN.1 Compiler

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attrs;

mod symbol;

mod per;

mod utils;

/// APER Codec Derive Macro support.
#[proc_macro_derive(AperCodec, attributes(asn))]
pub fn derive_aper_codec(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let codec_params = codec_params_or_err(&ast);
    if let Ok(codec_params) = codec_params {
        per::generate_codec(&ast, &codec_params, true)
    } else {
        codec_params.err().unwrap().to_compile_error().into()
    }
}

/// UPER Codec Derive Macro support.
#[proc_macro_derive(UperCodec, attributes(asn))]
pub fn derive_uper_codec(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let codec_params = codec_params_or_err(&ast);
    if let Ok(codec_params) = codec_params {
        per::generate_codec(&ast, &codec_params, false)
    } else {
        codec_params.err().unwrap().to_compile_error().into()
    }
}

fn codec_params_or_err(ast: &DeriveInput) -> Result<attrs::TyCodecParams, syn::Error> {
    let codec_params = attrs::parse_ty_meta_as_codec_params(&ast.attrs);
    if codec_params.is_err() {
        return Err(codec_params.err().unwrap());
    }

    let codec_params = codec_params.unwrap();
    if codec_params.attr.is_none() {
        return Err(syn::Error::new_spanned(
            ast,
            "Missing attribute 'asn' for the struct.",
        ));
    }

    if codec_params.ty.is_none() {
        return Err(syn::Error::new_spanned(
            codec_params.attr,
            "Missing parameter 'type' for the attribute.",
        ));
    }

    Ok(codec_params)
}
