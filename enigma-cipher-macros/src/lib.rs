extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(RotorTools)]
pub fn rotor_encode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_rotor_encode(&ast)
}

fn impl_rotor_encode(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl RotorTools for #name {
            fn advance(&mut self)  {
                let step = 1;

                self.cur_offset = match self.cur_offset + step > 25 {
                    true  => self.cur_offset + step - 25,
                    false => self.cur_offset + step,
                }
            }

            fn position(&self) -> char {
                _shift_char_offset(self.ring_setting, self.get_offset() * -1)
            }

            fn get_offset(&self) -> i8 {
                if self.cur_offset == self.init_offset {
                    return 0;
                } else if self.cur_offset < self.init_offset {
                    return (self.init_offset - self.cur_offset) as i8;
                } else {
                    return ((self.init_offset + 26) - self.cur_offset) as i8;
                }
            }
        }
    };

    gen.into()
}
