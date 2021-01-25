// src/rotors.rs
//
// Copyright (c) 2018
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms

use enigma_cipher_macros::RotorEncode;

fn _calculate_start_offset(input: char) -> u8 {
    (input as u8) - 65
}

fn _shift_char_offset(input: char, offset: i8) -> char {
    let input_val = input as i8;
    let shifted: i8 = input_val + offset;

    if shifted > 90 {
        return ((shifted - 26) as u8) as char;
    } else if shifted < 65 {
        return ((shifted + 26) as u8) as char;
    } else {
        return (shifted as u8) as char;
    }
}

pub trait RotorEncode {
    fn new(ring_setting: char, init_position: char) -> Self;
    fn transpose_in(&self, input: char) -> char;
    fn transpose_out(&self, input: char) -> char;
    fn advance(&mut self);
    fn position(&self) -> char;
    fn get_offset(&self) -> i8;
    fn at_notch(&self) -> bool;
}

pub trait Reflector {
    fn transpose(&self, input: char) -> char;
}

pub struct ReflectorB;

impl Reflector for ReflectorB {
    fn transpose(&self, input: char) -> char {
        match input {
            'A' => 'Y',
            'B' => 'R',
            'C' => 'U',
            'D' => 'H',
            'E' => 'Q',
            'F' => 'S',
            'G' => 'L',
            'H' => 'D',
            'I' => 'P',
            'J' => 'X',
            'K' => 'N',
            'L' => 'G',
            'M' => 'O',
            'N' => 'K',
            'O' => 'M',
            'P' => 'I',
            'Q' => 'E',
            'R' => 'B',
            'S' => 'F',
            'T' => 'Z',
            'U' => 'C',
            'V' => 'W',
            'W' => 'V',
            'X' => 'J',
            'Y' => 'A',
            'Z' => 'T',
            _   => ' ',
        }
    }
}

#[derive(RotorEncode)]
#[key_ordering(EKMFLGDQVZNTOWYHXUSPAIBRCJ)]
#[notches(Q)]
pub struct RotorI {
    ring_setting: char,
    init_offset: u8,
    cur_offset: u8,
}

#[derive(RotorEncode)]
#[key_ordering(AJDKSIRUXBLHWTMCQGZNPYFVOE)]
#[notches(E)]
pub struct RotorII {
    ring_setting: char,
    init_offset: u8,
    cur_offset: u8,
}

#[derive(RotorEncode)]
#[key_ordering(BDFHJLCPRTXVZNYEIWGAKMUSQO)]
#[notches(V)]
pub struct RotorIII {
    ring_setting: char,
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
