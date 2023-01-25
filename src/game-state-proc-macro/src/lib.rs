extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GameStateImpl)]
pub fn game_state_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics GameState for RefCell<#name> #ty_generics #where_clause {
            fn get_input_options(&self) -> Ref<Vec<InputOption>> {
                Ref::map(self.borrow(), |g_state| &g_state.input_options)
            }
            fn get_header(&self) -> String {
                self.borrow().header.clone()
            }
            fn get_body(&self) -> Option<String> {
                self.borrow().body.clone()
            }
            fn get_init_fn(&self) -> Option<fn(&mut GlobalGameState)> {
                self.borrow().init
            }
        }
    };

    TokenStream::from(expanded)
}
