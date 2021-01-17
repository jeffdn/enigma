// src/rotors.rs
//
// Copyright (c) 2018
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms

fn _calculate_start_offset(ring_setting: char, init_position: char) -> u8 {
    // match start {
    //     'A' => ('Z' as u8) - 65,
    //     _   => (start as u8) - 65,
    // }
    let ring_offset = (ring_setting as u8) - 65;
    let init_offset = (init_position as u8) - 65;
    _get_offset(ring_offset, init_offset)
}

fn _calculate_input_offset(input: char, offset: u8) -> char {
    let input_val: u8 = input as u8;
    let input_pos: u8 = match (input_val + offset) > 90 {
        true    => ((input_val + offset) - 26),
        false   => input_val + offset,
    };

    input_pos as char
}

fn _calculate_output_offset(input: char, offset: u8) -> char {
    let input_val: i8 = input as i8;
    let offset: i8 = offset as i8 * -1;
    // let offset: u8 = ((offset as i8 * -1) + 25) as u8;
    let input_pos: i8 = match (input_val + offset) < 65 {
        true    => ((input_val + offset) + 26),
        false   => input_val + offset,
    };

    (input_pos as u8) as char
}

fn _new_offset(cur_offset: u8) -> u8 {
    let step = 1;
    println!("{} + {} = {}", cur_offset, step, cur_offset + step);
    match cur_offset + step > 25 {
        true  => cur_offset + step - 25,
        false => cur_offset + step,
    }
}

fn _get_offset(init_offset: u8, cur_offset: u8) -> u8 {
    match init_offset + cur_offset > 25 {
        true  => init_offset + cur_offset - 25,
        false => init_offset + cur_offset,
    }
}

// pub struct Rotor<T> {
//     rotor: T,
// }
//
// impl<T: RotorEncode> Rotor<T> {
//     pub fn new(rotor: T) -> Self {
//         Rotor {
//             rotor: rotor,
//         }
//     }
//
//     pub fn transpose_in(&mut self, input: char) -> char {
//         self.rotor.transpose_in(input)
//     }
//
//     pub fn advance(&mut self) -> bool {
//         self.rotor.advance(extra_step)
//     }
// }

pub trait RotorEncode {
    fn new(ring_setting: char, init_position: char) -> Self;
    fn transpose_in(&self, input: char) -> char;
    fn transpose_out(&self, input: char) -> char;
    fn at_notch(&self) -> bool;
    fn advance(&mut self);
    fn position(&self) -> char;
    fn get_offset(&self) -> u8;
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

pub struct RotorI {
    ring_setting: char,
    init_offset: u8,
    cur_offset: u8,
}

impl RotorEncode for RotorI {
    fn new(ring_setting: char, init_position: char) -> Self {
        Self {
            ring_setting: ring_setting,
            init_offset: 0, //_calculate_start_offset(ring_setting),
            cur_offset: _calculate_start_offset(ring_setting, init_position),
        }
    }

    fn transpose_in(&self, input: char) -> char {
        let offset_input = _calculate_input_offset(input, self.get_offset());

        let computed = match offset_input as char {
            'A' => 'E',
            'B' => 'K',
            'C' => 'M',
            'D' => 'F',
            'E' => 'L',
            'F' => 'G',
            'G' => 'D',
            'H' => 'Q',
            'I' => 'V',
            'J' => 'Z',
            'K' => 'N',
            'L' => 'T',
            'M' => 'O',
            'N' => 'W',
            'O' => 'Y',
            'P' => 'H',
            'Q' => 'X',
            'R' => 'U',
            'S' => 'S',
            'T' => 'P',
            'U' => 'A',
            'V' => 'I',
            'W' => 'B',
            'X' => 'R',
            'Y' => 'C',
            'Z' => 'J',
            _   => ' ',
        };

        _calculate_output_offset(computed, self.get_offset())
    }

    fn transpose_out(&self, input: char) -> char {
        let offset_input = _calculate_input_offset(input, self.get_offset());

        let computed = match offset_input as char {
            'E' => 'A',
            'K' => 'B',
            'M' => 'C',
            'F' => 'D',
            'L' => 'E',
            'G' => 'F',
            'D' => 'G',
            'Q' => 'H',
            'V' => 'I',
            'Z' => 'J',
            'N' => 'K',
            'T' => 'L',
            'O' => 'M',
            'W' => 'N',
            'Y' => 'O',
            'H' => 'P',
            'X' => 'Q',
            'U' => 'R',
            'S' => 'S',
            'P' => 'T',
            'A' => 'U',
            'I' => 'V',
            'B' => 'W',
            'R' => 'X',
            'C' => 'Y',
            'J' => 'Z',
            _   => ' ',
        };

        _calculate_output_offset(computed, self.get_offset())
    }

    fn at_notch(&self) -> bool {
        println!("rotor i   - offset: {}", self.get_offset());
        (65 + self.get_offset()) as char == 'Q'
    }

    fn advance(&mut self)  {
        self.cur_offset = _new_offset(self.get_offset());
    }

    fn position(&self) -> char {
        _calculate_input_offset(self.ring_setting, self.get_offset())
    }

    fn get_offset(&self) -> u8 {
        _get_offset(self.init_offset, self.cur_offset)
    }
}

pub struct RotorII {
    ring_setting: char,
    init_offset: u8,
    cur_offset: u8,
}

impl RotorEncode for RotorII {
    fn new(ring_setting: char, init_position: char) -> Self {
        RotorII {
            ring_setting: ring_setting,
            init_offset: 0, //_calculate_start_offset(ring_setting),
            cur_offset: _calculate_start_offset(ring_setting, init_position),
        }
    }

    fn transpose_in(&self, input: char) -> char {
        let offset_input = _calculate_input_offset(input, self.get_offset());

        let computed = match offset_input as char {
            'A' => 'A',
            'B' => 'J',
            'C' => 'D',
            'D' => 'K',
            'E' => 'S',
            'F' => 'I',
            'G' => 'R',
            'H' => 'U',
            'I' => 'X',
            'J' => 'B',
            'K' => 'L',
            'L' => 'H',
            'M' => 'W',
            'N' => 'T',
            'O' => 'M',
            'P' => 'C',
            'Q' => 'Q',
            'R' => 'G',
            'S' => 'Z',
            'T' => 'N',
            'U' => 'P',
            'V' => 'Y',
            'W' => 'F',
            'X' => 'V',
            'Y' => 'O',
            'Z' => 'E',
            _   => ' ',
        };

        _calculate_output_offset(computed, self.get_offset())
    }

    fn transpose_out(&self, input: char) -> char {
        let offset_input = _calculate_input_offset(input, self.get_offset());

        let computed = match offset_input as char {
            'A' => 'A',
            'J' => 'B',
            'D' => 'C',
            'K' => 'D',
            'S' => 'E',
            'I' => 'F',
            'R' => 'G',
            'U' => 'H',
            'X' => 'I',
            'B' => 'J',
            'L' => 'K',
            'H' => 'L',
            'W' => 'M',
            'T' => 'N',
            'M' => 'O',
            'C' => 'P',
            'Q' => 'Q',
            'G' => 'R',
            'Z' => 'S',
            'N' => 'T',
            'P' => 'U',
            'Y' => 'V',
            'F' => 'W',
            'V' => 'X',
            'O' => 'Y',
            'E' => 'Z',
            _   => ' ',
        };

        _calculate_output_offset(computed, self.get_offset())
    }

    fn at_notch(&self) -> bool {
        println!("rotor ii  - offset: {}", self.get_offset());
        (65 + self.get_offset()) as char == 'E'
    }

    fn advance(&mut self)  {
        self.cur_offset = _new_offset(self.get_offset());
    }

    fn position(&self) -> char {
        _calculate_input_offset(self.ring_setting, self.get_offset())
    }

    fn get_offset(&self) -> u8 {
        _get_offset(self.init_offset, self.cur_offset)
    }
}

pub struct RotorIII {
    ring_setting: char,
    init_offset: u8,
    cur_offset: u8,
}

impl RotorEncode for RotorIII {
    fn new(ring_setting: char, init_position: char) -> Self {
        Self {
            ring_setting: ring_setting,
            init_offset: 0, //_calculate_start_offset(ring_setting),
            cur_offset: _calculate_start_offset(ring_setting, init_position),
        }
    }

    fn transpose_in(&self, input: char) -> char {
        let offset_input = _calculate_input_offset(input, self.get_offset());

        let computed = match offset_input as char {
            'A' => 'B',
            'B' => 'D',
            'C' => 'F',
            'D' => 'H',
            'E' => 'J',
            'F' => 'L',
            'G' => 'C',
            'H' => 'P',
            'I' => 'R',
            'J' => 'T',
            'K' => 'X',
            'L' => 'V',
            'M' => 'Z',
            'N' => 'N',
            'O' => 'Y',
            'P' => 'E',
            'Q' => 'I',
            'R' => 'W',
            'S' => 'G',
            'T' => 'A',
            'U' => 'K',
            'V' => 'M',
            'W' => 'U',
            'X' => 'S',
            'Y' => 'Q',
            'Z' => 'O',
            _   => ' ',
        };

        _calculate_output_offset(computed, self.get_offset())
    }

    fn transpose_out(&self, input: char) -> char {
        let offset_input = _calculate_input_offset(input, self.get_offset());

        let computed = match offset_input as char {
            'B' => 'A',
            'D' => 'B',
            'F' => 'C',
            'H' => 'D',
            'J' => 'E',
            'L' => 'F',
            'C' => 'G',
            'P' => 'H',
            'R' => 'I',
            'T' => 'J',
            'X' => 'K',
            'V' => 'L',
            'Z' => 'M',
            'N' => 'N',
            'Y' => 'O',
            'E' => 'P',
            'I' => 'Q',
            'W' => 'R',
            'G' => 'S',
            'A' => 'T',
            'K' => 'U',
            'M' => 'V',
            'U' => 'W',
            'S' => 'X',
            'Q' => 'Y',
            'O' => 'Z',
            _   => ' ',
        };

        _calculate_output_offset(computed, self.get_offset())
    }

    fn at_notch(&self) -> bool {
        println!("rotor iii - offset: {}", self.get_offset());
        (65 + self.get_offset()) as char == 'V'
    }

    fn advance(&mut self)  {
        self.cur_offset = _new_offset(self.get_offset());
    }

    fn position(&self) -> char {
        _calculate_input_offset(self.ring_setting, self.get_offset())
    }

    fn get_offset(&self) -> u8 {
        _get_offset(self.init_offset, self.cur_offset)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_offset() {
        assert_eq!('B', _calculate_input_offset('A', 1));
        assert_eq!('A', _calculate_input_offset('Z', 1));
        assert_eq!('A', _calculate_input_offset('S', 8));
        assert_eq!('F', _calculate_input_offset('B', 4));
    }

    #[test]
    fn test_output_offset() {
        assert_eq!('Z', _calculate_output_offset('A', 1));
        assert_eq!('Y', _calculate_output_offset('Z', 1));
        assert_eq!('K', _calculate_output_offset('S', 8));
        assert_eq!('X', _calculate_output_offset('B', 4));
    }
}
