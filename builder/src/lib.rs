use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_derive_macro(&ast)
}

fn impl_derive_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
    pub struct CommandBuilder {
        executable: Option<String>,
        args: Option<Vec<String>>,
        env: Option<Vec<String>>,
        current_dir: Option<String>,
    }
            impl #name {
                pub fn builder() -> CommandBuilder {
                    CommandBuilder {
                        executable: None,
                        args: None,
                        env: None,
                        current_dir: None
                    }
                }
            }
        };
    gen.into()
}
