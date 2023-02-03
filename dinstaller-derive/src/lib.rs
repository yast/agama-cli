use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(DInstallerAttributes)]
pub fn dinstaller_attributes_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("only structs are supported"),
    };
    let field_name = fields.iter().map(|field| &field.ident);
    let name = input.ident;
    let expanded = quote! {
        impl Attributes for #name {
            fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
                match attr {
                    #(stringify!(#field_name) => self.#field_name = value.try_into()?,)*
                    _ => return Err("unknown attribute")
                };
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
