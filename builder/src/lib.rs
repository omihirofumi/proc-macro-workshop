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
        impl CommandBuilder {
            fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }
            fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }
            fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }
            fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }
            fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(
                    Command {
                        executable: self.executable.take().ok_or("executable is not set")?,
                        args: self.args.take().ok_or("args is not set")?,
                        env: self.env.take().ok_or("env is not set")?,
                        current_dir: self.current_dir.take().ok_or("current_dir is not set")?
                    }
                )
            }
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
