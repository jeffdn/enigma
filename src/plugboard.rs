// src/lib.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

use std::collections::HashMap;

pub type Plugboard = HashMap<char, char>;

#[macro_export]
macro_rules! plugboard {
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
            Some(intermediate)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let board = plugboard! { 'A' => 'E' }.unwrap();

        assert_eq!(board.get(&'A'), Some(&'E'));
        assert_eq!(board.get(&'E'), Some(&'A'));
    }

    #[test]
    fn test_many() {
        let board = plugboard! {
            'A' => 'E',
            'F' => 'J',
            'M' => 'G'
        }.unwrap();

        assert_eq!(board.get(&'A'), Some(&'E'));
        assert_eq!(board.get(&'E'), Some(&'A'));
        assert_eq!(board.get(&'F'), Some(&'J'));
        assert_eq!(board.get(&'J'), Some(&'F'));
        assert_eq!(board.get(&'M'), Some(&'G'));
        assert_eq!(board.get(&'G'), Some(&'M'));
    }

    #[test]
    #[should_panic(expected = "'É' is not a valid character, must be A-Z")]
    fn test_invalid_left() {
        let _board = plugboard! { 'É' => 'A' };
    }

    #[test]
    #[should_panic(expected = "'É' is not a valid character, must be A-Z")]
    fn test_invalid_right() {
        let _board = plugboard! { 'A' => 'É' };
    }

    #[test]
    #[should_panic(expected = "Neither 'Ö' nor 'É' are valid characters, must be A-Z")]
    fn test_both_invalid() {
        let _board = plugboard! { 'Ö' => 'É' };
    }

    #[test]
    #[should_panic(expected = "'A' already wired to plugboard!")]
    fn test_left_already_wired() {
        let _board = plugboard! {
            'A' => 'F',
            'A' => 'E'
        };
    }

    #[test]
    #[should_panic(expected = "'F' already wired to plugboard!")]
    fn test_right_already_wired() {
        let _board = plugboard! {
            'A' => 'F',
            'F' => 'E'
        };
    }
}
