//! Helper functions for deriving `EthAbiType`

use ethers_core::macros::ethers_core_crate;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput};

/// Generates the `AbiEncode` + `AbiDecode` implementation
pub fn derive_codec_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let ethers_core = ethers_core_crate();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let predicates = where_clause.map(|c| &c.predicates);

    let mk_predicates = |predicate| match &input.data {
        syn::Data::Struct(strct) => strct
            .fields
            .iter()
            .map(|f| &f.ty)
            .map(|ty| quote_spanned!(ty.span()=> #ty: #predicate))
            .collect::<Vec<_>>(),
        syn::Data::Enum(enm) => enm
            .variants
            .iter()
            .flat_map(|v| v.fields.iter().map(|f| &f.ty))
            .map(|ty| quote_spanned!(ty.span()=> #ty: #predicate))
            .collect::<Vec<_>>(),
        syn::Data::Union(_) => panic!("not supported"),
    };

    let encode_predicates =
        mk_predicates(quote!(#ethers_core::abi::AbiEncode + #ethers_core::abi::Tokenizable));
    let decode_predicates = mk_predicates(
        quote!(#ethers_core::abi::AbiDecode + #ethers_core::abi::AbiType + #ethers_core::abi::Tokenizable),
    );

    quote! {
        impl #impl_generics #ethers_core::abi::AbiDecode for #name #ty_generics
        where
            #(#decode_predicates,)*
            #predicates
        {
            fn decode(bytes: impl AsRef<[u8]>) -> ::core::result::Result<Self, #ethers_core::abi::AbiError> {
                fn _decode #ty_generics (bytes: &[u8]) -> ::core::result::Result<#name #ty_generics, #ethers_core::abi::AbiError>
                where
                    #(#decode_predicates,)*
                    #predicates
                {
                    let #ethers_core::abi::ParamType::Tuple(params) =
                        <#name #ty_generics as #ethers_core::abi::AbiType>::param_type() else { unreachable!() };
                    let min_len: usize = params.iter().map(#ethers_core::abi::minimum_size).sum();
                    if bytes.len() < min_len {
                        Err(#ethers_core::abi::AbiError::DecodingError(#ethers_core::abi::ethabi::Error::InvalidData))
                    } else {
                        let tokens = #ethers_core::abi::decode(&params, bytes)?;
                        let tuple = #ethers_core::abi::Token::Tuple(tokens);
                        let this = <#name #ty_generics as #ethers_core::abi::Tokenizable>::from_token(tuple)?;
                        Ok(this)
                    }
                }

                _decode(bytes.as_ref())
            }
        }

        impl #impl_generics #ethers_core::abi::AbiEncode for #name #ty_generics
        where
            #(#encode_predicates,)*
            #predicates
        {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = #ethers_core::abi::Tokenize::into_tokens(self);
                #ethers_core::abi::encode(&tokens)
            }
        }
    }
}
