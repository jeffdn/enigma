// src/rotors.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

use enigma_macros::RotorEncode;

fn _apply_offset(shifted: i8) -> char {
    if shifted > 90 {
        ((shifted - 26) as u8) as char
    } else if shifted < 65 {
        ((shifted + 26) as u8) as char
    } else {
        (shifted as u8) as char
    }
}

pub trait RotorEncode {
    fn new(ring_setting: char, init_position: char) -> Self;
    fn _shift_input(&self, input: char) -> char;
    fn _shift_output(&self, output: char) -> char;
    fn transpose_in(&self, input: char) -> char;
    fn transpose_out(&self, input: char) -> char;
    fn advance(&mut self);
    fn ring_setting(&self) -> char;
    fn init_position(&self) -> char;
    fn position(&self) -> char;
    fn get_offset(&self) -> i8;
    fn at_notch(&self) -> bool;
}

#[derive(RotorEncode)]
#[key_ordering(EKMFLGDQVZNTOWYHXUSPAIBRCJ)]
#[notches(Q)]
pub struct RotorI {
    ring_setting: char,
    init_position: char,
    init_offset: u8,
    cur_offset: u8,
}

#[derive(RotorEncode)]
#[key_ordering(AJDKSIRUXBLHWTMCQGZNPYFVOE)]
#[notches(E)]
pub struct RotorII {
    ring_setting: char,
    init_position: char,
    init_offset: u8,
    cur_offset: u8,
}

#[derive(RotorEncode)]
#[key_ordering(BDFHJLCPRTXVZNYEIWGAKMUSQO)]
#[notches(V)]
pub struct RotorIII {
    ring_setting: char,
    init_position: char,
    init_offset: u8,
    cur_offset: u8,
}

#[derive(RotorEncode)]
#[key_ordering(ESOVPZJAYQUIRHXLNFTGKDCMWB)]
#[notches(J)]
pub struct RotorIV {
    ring_setting: char,
    init_position: char,
    init_offset: u8,
    cur_offset: u8,
}

#[derive(RotorEncode)]
#[key_ordering(VZBRGITYUPSDNHLXAWMJQOFECK)]
#[notches(Z)]
pub struct RotorV {
    ring_setting: char,
    init_position: char,
    init_offset: u8,
    cur_offset: u8,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transpose_in() {
        let rotor = RotorI::new('A', 'A');
        assert_eq!(rotor.transpose_in('A'), 'E');

        let rotor = RotorI::new('A', 'B');
        assert_eq!(rotor.transpose_in('A'), 'J');

        let rotor = RotorI::new('B', 'A');
        assert_eq!(rotor.transpose_in('A'), 'K');

        let rotor = RotorI::new('F', 'Y');
        assert_eq!(rotor.transpose_in('A'), 'W');
    }

    #[test]
    fn test_transpose_out() {
        let rotor = RotorI::new('A', 'A');
        assert_eq!(rotor.transpose_out('E'), 'A');

        let rotor = RotorI::new('A', 'B');
        assert_eq!(rotor.transpose_out('J'), 'A');

        let rotor = RotorI::new('B', 'A');
        assert_eq!(rotor.transpose_out('K'), 'A');

        let rotor = RotorI::new('F', 'Y');
        assert_eq!(rotor.transpose_out('W'), 'A');
    }
}
