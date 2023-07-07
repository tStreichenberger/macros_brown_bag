use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn change_name(args: TokenStream, input: TokenStream) -> TokenStream {
    let ident = syn::parse_macro_input!(args as syn::Ident);

    let mut func = syn::parse_macro_input!(input as syn::ItemFn);

    func.sig.ident = ident;

    quote! {
        #func
    }.into()
}



#[proc_macro_derive(Clone)]
pub fn clone(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    let struct_name = input.ident;

    let field_clones: Vec<_> = input.fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        quote! { #field_name: self.#field_name.clone(), }
    }).collect();

    quote! {
        impl Clone for #struct_name {
            fn clone(&self) -> Self {
                Self {
                    #(#field_clones)*
                }
            }
        }
    }.into()
}