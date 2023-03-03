use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(DInstallerAttributes, attributes(collection))]
pub fn dinstaller_attributes_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("only structs are supported"),
    };

    let (collection, scalar): (Vec<_>, Vec<_>) = fields
        .iter()
        .partition(|f| f.attrs.iter().any(|a| a.path.is_ident("collection")));

    let field_name = scalar.iter().map(|field| &field.ident);
    let name = input.ident;
    let set_fn = quote! {
        fn set(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
            match attr {
                #(stringify!(#field_name) => self.#field_name = value.try_into()?,)*
                _ => return Err("unknown attribute")
            };
            Ok(())
        }
    };

    let mut add_fn = quote! {};

    if !collection.is_empty() {
        let field_name = collection.iter().map(|field| &field.ident);
        add_fn = quote! {
            fn add(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
                match attr {
                    #(stringify!(#field_name) => self.#field_name.push(value.try_into()?),)*
                    _ => return Err("unknown attribute")
                };
                Ok(())
            }
        };
    }

    let expanded = quote! {
        impl Attributes for #name {
            #set_fn
            #add_fn
        }
    };

    TokenStream::from(expanded)
}
