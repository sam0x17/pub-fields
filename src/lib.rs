use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Nothing, parse_macro_input, parse_quote, ItemStruct};

#[proc_macro_attribute]
pub fn pub_fields(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    parse_macro_input!(attr as Nothing);
    let mut item_struct = parse_macro_input!(tokens as ItemStruct);
    for field in &mut item_struct.fields {
        field.vis = match &field.vis {
            syn::Visibility::Public(p) => syn::Visibility::Public(*p),
            syn::Visibility::Restricted(res) => syn::Visibility::Restricted(res.clone()),
            syn::Visibility::Inherited => parse_quote!(pub),
        };
    }
    item_struct.to_token_stream().into()
}
