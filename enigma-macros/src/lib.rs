// src/enigma-cipher-macros/lib.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn;

fn check_keyspace(key_tokens: &String) {
    let mut keyspace: Vec<char> = key_tokens.chars().collect();
    let mut expected: Vec<char> = ('A'..='Z').collect();

    expected.sort();
    keyspace.sort();

    assert_eq!(expected, keyspace, "Expected 26 unique characters in #[key_ordering(...)]");
}

fn extract_attribute(tokens: &proc_macro2::TokenStream) -> String {
    let attr_str = &tokens.to_string();
    let to_trim: &[_] = &['(', ')'];

    attr_str.trim_matches(to_trim).into()
}

fn generate_key_mappings(key_ordering: String) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut transpose_in: proc_macro2::TokenStream = quote!();
    let mut transpose_out: proc_macro2::TokenStream = quote!();

    check_keyspace(&key_ordering);

    for (ordering_char, mapped_char) in key_ordering.chars().zip('A'..='Z') {
        transpose_in.extend(quote! {
            #mapped_char => #ordering_char,
        });
        transpose_out.extend(quote! {
            #ordering_char => #mapped_char,
        });
    }

    (transpose_in, transpose_out)
}

#[proc_macro_derive(RotorEncode, attributes(key_ordering, notches))]
pub fn rotor_encode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_rotor_encode(&ast)
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

    let (transpose_in, transpose_out) = generate_key_mappings(key_ordering);

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
                    init_position: init_position,
                    init_offset: (ring_setting as u8) - 65,
                    cur_offset: (init_position as u8) - 65,
                }
            }

            fn _shift_input(&self, input: char) -> char {
                let offset: i8 = self.get_offset() * -1;
                let input_val = input as i8;

                _apply_offset(input_val + offset)
            }

            fn _shift_output(&self, output: char) -> char {
                let offset: i8 = self.get_offset();
                let output_val = output as i8;

                _apply_offset(output_val + offset)
            }

            fn transpose_in(&self, input: char) -> char {
                let computed = match self._shift_input(input) {
                    #transpose_in
                    _  => ' ',
                };

                self._shift_output(computed)
            }

            fn transpose_out(&self, input: char) -> char {
                let computed = match self._shift_input(input) {
                    #transpose_out
                    _  => ' ',
                };

                self._shift_output(computed)
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

            fn ring_setting(&self) -> char {
                self.ring_setting
            }

            fn init_position(&self) -> char {
                self.init_position
            }

            fn position(&self) -> char {
                self._shift_input(self.ring_setting)
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

#[proc_macro_derive(Reflector, attributes(key_ordering))]
pub fn reflector_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_reflector(&ast)
}

fn impl_reflector(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut key_ordering: String = "empty".into();

    for attr in ast.attrs.iter() {
        let attr_name = attr.path.get_ident().unwrap().to_string();
        match &*attr_name {
            "key_ordering" => key_ordering = extract_attribute(&attr.tokens),
            _ => {},
        };
    }

    let (transpose, _) = generate_key_mappings(key_ordering);

    let gen = quote! {
        impl Reflector for #name {
            fn transpose(&self, input: char) -> char {
                match input {
                    #transpose
                    _  => ' ',
                }
            }
        }
    };

    gen.into()
}

#[cfg(test)]
mod tests {
    use super::check_keyspace;

    #[test]
    #[should_panic(expected = "Expected 26 unique characters in #[key_ordering(...)]")]
    fn test_check_keyspace_too_short() {
        let input: String = ('A'..='M').collect();
        check_keyspace(&input);
    }

    #[test]
    #[should_panic(expected = "Expected 26 unique characters in #[key_ordering(...)]")]
    fn test_check_keyspace_duplicate() {
        let mut input: String = ('A'..='Z').collect();
        input.push('Z');

        // But has duplicates!
        check_keyspace(&input);
    }
}
