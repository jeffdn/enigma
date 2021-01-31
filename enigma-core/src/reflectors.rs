// src/rotors.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

use enigma_macros::Reflector;

pub trait Reflector {
    fn transpose(&self, input: char) -> char;
}

#[derive(Reflector)]
#[key_ordering(EJMZALYXVBWFCRQUONTSPIKHGD)]
pub struct ReflectorA;

#[derive(Reflector)]
#[key_ordering(YRUHQSLDPXNGOKMIEBFZCWVJAT)]
pub struct ReflectorB;

#[derive(Reflector)]
#[key_ordering(FVPJIAOYEDRZXWGCTKUQSBNMHL)]
pub struct ReflectorC;
