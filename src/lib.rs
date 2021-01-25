// src/lib.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

pub mod rotors;

use rotors::{Reflector, RotorEncode};

pub trait Enigma {
    fn keypress(&mut self, input: char) -> char;
    fn settings(&self) -> Vec<char>;
}

pub struct ArmyEnigma<A, B, C, D> {
    rotor1: A,
    rotor2: B,
    rotor3: C,
    reflector: D,
}

impl<A: RotorEncode, B: RotorEncode, C: RotorEncode, D: Reflector> ArmyEnigma<A, B, C, D> {
    pub fn new(rotor1: A, rotor2: B, rotor3: C, reflector: D) -> Self {
        ArmyEnigma {
            rotor1: rotor1,
            rotor2: rotor2,
            rotor3: rotor3,
            reflector: reflector,
        }
    }
}

impl<A: RotorEncode, B: RotorEncode, C: RotorEncode, D: Reflector> Enigma for ArmyEnigma<A, B, C, D> {
    fn keypress(&mut self, input: char) -> char {
        let right_at_notch = self.rotor3.at_notch();
        let middle_at_notch = self.rotor2.at_notch();

        self.rotor3.advance();

        if right_at_notch {
            self.rotor2.advance();
        }

        if middle_at_notch {
            self.rotor2.advance();
            self.rotor1.advance();
        }

        let output = self.rotor3.transpose_in(input);
        let output = self.rotor2.transpose_in(output);
        let output = self.rotor1.transpose_in(output);
        let output = self.reflector.transpose(output);
        let output = self.rotor1.transpose_out(output);
        let output = self.rotor2.transpose_out(output);
        let output = self.rotor3.transpose_out(output);

        output
    }

    fn settings(&self) -> Vec<char> {
        vec![self.rotor1.position(), self.rotor2.position(), self.rotor3.position()]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rotors::*;

    #[test]
    fn test_simple() {
        let mut machine = ArmyEnigma::new(
            RotorI::new('A', 'A'),
            RotorII::new('A', 'A'),
            RotorIII::new('A', 'A'),
            ReflectorB{},
        );

        let input: Vec<char> = vec!['A', 'A', 'A', 'A', 'A'];
        let expected: Vec<char> = vec!['B', 'D', 'Z', 'G', 'O'];
        let output: Vec<char> = input.into_iter().map(|in_char| machine.keypress(in_char)).collect();

        assert_eq!(expected, output);

        let expected_settings = vec!['A', 'A', 'F'];
        assert_eq!(expected_settings, machine.settings());
    }

    #[test]
    fn test_enigma() {
        let mut machine = ArmyEnigma::new(
            RotorI::new('A', 'A'),
            RotorII::new('A', 'A'),
            RotorIII::new('A', 'A'),
            ReflectorB{},
        );

        let input: Vec<char> = vec!['E', 'N', 'I', 'G', 'M', 'A'];
        let expected: Vec<char> = vec!['F', 'Q', 'G', 'A', 'H', 'W'];
        let output: Vec<char> = input.into_iter().map(|in_char| machine.keypress(in_char)).collect();

        assert_eq!(expected, output);

        let expected_settings = vec!['A', 'A', 'G'];
        assert_eq!(expected_settings, machine.settings());
    }

    #[test]
    fn test_homogenous_rotors() {
        let mut machine = ArmyEnigma::new(
            RotorI::new('A', 'A'),
            RotorI::new('A', 'A'),
            RotorI::new('A', 'A'),
            ReflectorB{},
        );

        let input: Vec<char> = vec!['A', 'A', 'A'];
        let expected: Vec<char> = vec!['U', 'O', 'T'];
        let output: Vec<char> = input.into_iter().map(|in_char| machine.keypress(in_char)).collect();

        assert_eq!(expected, output);

        let expected_settings = vec!['A', 'A', 'D'];
        assert_eq!(expected_settings, machine.settings());
    }

    #[test]
    fn test_turnover() {
        let mut machine = ArmyEnigma::new(
            RotorI::new('A', 'A'),
            RotorII::new('A', 'D'),
            RotorIII::new('A', 'U'),
            ReflectorB{},
        );

        let input: Vec<char> = vec!['A', 'A', 'A', 'A', 'A'];
        let expected: Vec<char> = vec!['E', 'Q', 'I', 'B', 'M'];
        let output: Vec<char> = input.into_iter().map(|in_char| machine.keypress(in_char)).collect();

        assert_eq!(expected, output);

        let expected_settings = vec!['B', 'F', 'Z'];
        assert_eq!(expected_settings, machine.settings());
    }

    #[test]
    fn test_gap_fog() {
        let mut machine = ArmyEnigma::new(
            RotorI::new('G', 'F'),
            RotorII::new('A', 'O'),
            RotorIII::new('P', 'G'),
            ReflectorB{},
        );

        let input: Vec<char> = vec!['A', 'D', 'V', 'A', 'N', 'C', 'E', 'M', 'I', 'N', 'S', 'K'];
        let expected: Vec<char> = vec!['P', 'X', 'B', 'U', 'Y', 'V', 'U', 'G', 'E', 'G', 'C', 'I'];
        let output: Vec<char> = input.into_iter().map(|in_char| machine.keypress(in_char)).collect();

        assert_eq!(expected, output);

        let expected_settings = vec!['F', 'O', 'S'];
        assert_eq!(expected_settings, machine.settings());
    }

    #[test]
    fn test_bbb_fog() {
        let mut machine = ArmyEnigma::new(
            RotorI::new('B', 'F'),
            RotorII::new('B', 'O'),
            RotorIII::new('B', 'G'),
            ReflectorB{},
        );

        let input: Vec<char> = vec!['A', 'D', 'V', 'A', 'N', 'C', 'E', 'M', 'I', 'N', 'S', 'K'];
        let expected: Vec<char> = vec!['Y', 'X', 'L', 'E', 'O', 'P', 'V', 'F', 'D', 'T', 'O', 'Y'];
        let output: Vec<char> = input.into_iter().map(|in_char| machine.keypress(in_char)).collect();

        assert_eq!(expected, output);

        let expected_settings = vec!['F', 'O', 'S'];
        assert_eq!(expected_settings, machine.settings());
    }

    #[test]
    fn test_ring_settings() {
        let mut machine = ArmyEnigma::new(
            RotorI::new('B', 'A'),
            RotorI::new('B', 'A'),
            RotorI::new('B', 'A'),
            ReflectorB{},
        );

        let input: Vec<char> = vec!['A', 'A', 'A'];
        let expected: Vec<char> = vec!['T', 'B', 'U'];
        let output: Vec<char> = input.into_iter().map(|in_char| machine.keypress(in_char)).collect();

        assert_eq!(expected, output);

        let expected_settings = vec!['A', 'A', 'D'];
        assert_eq!(expected_settings, machine.settings());
    }
}
