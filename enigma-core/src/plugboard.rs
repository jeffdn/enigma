// src/plugboard.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PlugboardError {
    InvalidCharacter((char, char)),
    CharacterAlreadyWired(char),
}

impl Error for PlugboardError { }
impl fmt::Display for PlugboardError {
    fn fmt(&self,  f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlugboardError::InvalidCharacter((l, r)) => write!(f, "'{}' or '{}' is not an uppercase ASCII letter", l, r),
            PlugboardError::CharacterAlreadyWired(c) => write!(f, "'{}' already wired to the board", c),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Plugboard(HashMap<char, char>);

impl Plugboard {
    pub fn new(pairs: &Vec<(char, char)>) -> Result<Self, PlugboardError> {
        let mut intermediate: HashMap<char, char> = HashMap::new();

        for (left, right) in pairs.clone().into_iter() {
            match (left, right) {
                ('A'..='Z', 'A'..='Z') => {},
                ('A'..='Z', _)         => return Err(PlugboardError::InvalidCharacter((left, right))),
                (_, 'A'..='Z')         => return Err(PlugboardError::InvalidCharacter((left, right))),
                (_, _)                 => return Err(PlugboardError::InvalidCharacter((left, right))),
            };

            if intermediate.contains_key(&left) {
                return Err(PlugboardError::CharacterAlreadyWired(left));
            }

            if intermediate.contains_key(&right) {
                return Err(PlugboardError::CharacterAlreadyWired(right));
            }

            intermediate.insert(left.clone(), right.clone());
            intermediate.insert(right.clone(), left.clone());
        }

        Ok(Self(intermediate))
    }

    pub fn from_map(intermediate: HashMap<char, char>) -> Self {
        Self(intermediate)
    }

    pub fn transpose(&self, input: char) -> char {
        match self.0.get(&input) {
            Some(ref c) => **c,
            None => input,
        }
    }
}

#[macro_export]
macro_rules! plugboard {
    () => {{ None }};
    ( $( $left:expr => $right:expr ),* ) => {
        {
            let mut intermediate = HashMap::new();
            $(
                match ($left, $right) {
                    ('A'..='Z', 'A'..='Z') => {},
                    ('A'..='Z', _)         => panic!(format!("{:?} is not a valid character, must be A-Z", $right)),
                    (_, 'A'..='Z')         => panic!(format!("{:?} is not a valid character, must be A-Z", $left)),
                    (_, _)                 => panic!(format!("Neither {:?} nor {:?} are valid characters, must be A-Z", $left, $right)),
                };

                assert!(!intermediate.contains_key(&$left), format!("{:?} already wired to plugboard!", $left));
                assert!(!intermediate.contains_key(&$right), format!("{:?} already wired to plugboard!", $right));
                intermediate.insert($left.clone(), $right.clone());
                intermediate.insert($right.clone(), $left.clone());
            )*

            Some(Plugboard::from_map(intermediate))
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macro_one() {
        let board = plugboard! { 'A' => 'E' }.unwrap();

        assert_eq!(board.transpose('A'), 'E');
        assert_eq!(board.transpose('E'), 'A');
        assert_eq!(board.transpose('Z'), 'Z');
    }

    #[test]
    fn test_macro_many() {
        let board = plugboard! {
            'A' => 'E',
            'F' => 'J',
            'M' => 'G'
        }.unwrap();

        assert_eq!(board.transpose('A'), 'E');
        assert_eq!(board.transpose('E'), 'A');
        assert_eq!(board.transpose('F'), 'J');
        assert_eq!(board.transpose('J'), 'F');
        assert_eq!(board.transpose('M'), 'G');
        assert_eq!(board.transpose('G'), 'M');
        assert_eq!(board.transpose('Z'), 'Z');
    }

    #[test]
    fn test_instantiation_errors() {
        let board = Plugboard::new(&vec![('É', 'A')]);

        assert!(board.is_err());
        assert_eq!(board, Err(PlugboardError::InvalidCharacter(('É', 'A'))));

        let board = Plugboard::new(&vec![('A', 'É')]);

        assert!(board.is_err());
        assert_eq!(board, Err(PlugboardError::InvalidCharacter(('A', 'É'))));

        let board = Plugboard::new(&vec![('Ö', 'É')]);

        assert!(board.is_err());
        assert_eq!(board, Err(PlugboardError::InvalidCharacter(('Ö', 'É'))));

        let board = Plugboard::new(&vec![('A', 'F'), ('A', 'E')]);

        assert!(board.is_err());
        assert_eq!(board, Err(PlugboardError::CharacterAlreadyWired('A')));

        let board = Plugboard::new(&vec![('A', 'F'), ('E', 'F')]);

        assert!(board.is_err());
        assert_eq!(board, Err(PlugboardError::CharacterAlreadyWired('F')));
    }

    #[test]
    #[should_panic(expected = "'É' is not a valid character, must be A-Z")]
    fn test_macro_invalid_left() {
        let _board = plugboard! { 'É' => 'A' };
    }

    #[test]
    #[should_panic(expected = "'É' is not a valid character, must be A-Z")]
    fn test_macro_invalid_right() {
        let _board = plugboard! { 'A' => 'É' };
    }

    #[test]
    #[should_panic(expected = "Neither 'Ö' nor 'É' are valid characters, must be A-Z")]
    fn test_macro_both_invalid() {
        let _board = plugboard! { 'Ö' => 'É' };
    }

    #[test]
    #[allow(unreachable_patterns)]
    #[should_panic(expected = "'A' already wired to plugboard!")]
    fn test_macro_left_already_wired() {
        let _board = plugboard! {
            'A' => 'F',
            'A' => 'E'
        };
    }

    #[test]
    #[allow(unreachable_patterns)]
    #[should_panic(expected = "'F' already wired to plugboard!")]
    fn test_macro_right_already_wired() {
        let _board = plugboard! {
            'A' => 'F',
            'E' => 'F'
        };
    }
}
