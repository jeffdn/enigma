extern crate proc_macro;

use std::collections::HashSet;
use std::iter::FromIterator;
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn;

#[proc_macro_derive(RotorEncode, attributes(key_ordering, notches))]
pub fn rotor_encode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_rotor_encode(&ast)
}

fn extract_attribute(tokens: &proc_macro2::TokenStream) -> String {
    let attr_str = &tokens.to_string();
    let to_trim: &[_] = &['(', ')'];

    attr_str.trim_matches(to_trim).into()
}

fn impl_rotor_encode(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut key_ordering: String = "empty".into();
    let mut notch_tokens: String = "empty".into();

    for attr in ast.attrs.iter() {
        let attr_name = attr.path.get_ident().unwrap().to_string();
        match &*attr_name {
            "key_ordering" => key_ordering = extract_attribute(&attr.tokens),
            "notches" => notch_tokens = extract_attribute(&attr.tokens),
            _ => {},
        };
    }

    let key_tokens: Vec<char> = key_ordering.chars().collect();
    let key_space: HashSet<char> = HashSet::from_iter(key_tokens.clone().into_iter());

    if key_space.len() != 26 {
        assert!(false, "Expected 26 unique characters in #[key_ordering(...)]");
    }

    let mut transpose_in: proc_macro2::TokenStream = quote!();
    let mut transpose_out: proc_macro2::TokenStream = quote!();

    for (i, x) in key_tokens.into_iter().enumerate() {
        let mapped_char = ((i + 65) as u8) as char;
        transpose_in.extend(quote! {
            #mapped_char => #x,
        });
        transpose_out.extend(quote! {
            #x => #mapped_char,
        });
    }

    let mut notch_map: proc_macro2::TokenStream = quote!();

    for x in notch_tokens.chars() {
        notch_map.extend(quote! {
            #x => true,
        });
    }

    let gen = quote! {
        impl RotorEncode for #name {
            fn new(ring_setting: char, init_position: char) -> Self {
                Self {
                    ring_setting: ring_setting,
                    init_offset: (ring_setting as u8) - 65,
                    cur_offset: (init_position as u8) - 65,
                }
            }

            fn transpose_in(&self, input: char) -> char {
                let offset_input = _shift_char_offset(input, self.get_offset() * -1);

                let computed = match offset_input as char {
                    #transpose_in
                    _  => ' ',
                };

                _shift_char_offset(computed, self.get_offset())
            }

            fn transpose_out(&self, input: char) -> char {
                let offset_input = _shift_char_offset(input, self.get_offset() * -1);

                let computed = match offset_input as char {
                    #transpose_out
                    _  => ' ',
                };

                _shift_char_offset(computed, self.get_offset())
            }

            fn at_notch(&self) -> bool {
                match (65 + self.cur_offset) as char {
                    #notch_map
                    _ => false,
                }
            }

            fn advance(&mut self)  {
                let step = 1;

                self.cur_offset = match self.cur_offset + step > 25 {
                    true => self.cur_offset + step - 25,
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