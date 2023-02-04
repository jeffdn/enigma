// src/ui/state.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

pub struct MachineState {
    pub machine_state: String,
    pub input_state: String,
    pub output_state: String,
}

impl MachineState {
    pub fn new(rotor_positions: &[char]) -> Self {
        Self {
            machine_state: MachineState::build_rotor_string(rotor_positions),
            input_state: "".into(),
            output_state: "".into(),
        }
    }

    pub fn update(&mut self, input: char, output: char, rotor_positions: &[char]) {
        self.machine_state = MachineState::build_rotor_string(rotor_positions);

        self.input_state.push_str(&format!("{input}"));
        self.output_state.push_str(&format!("{output}"));

        let raw_chars: Vec<char> = self.input_state.chars().filter(|x| *x != ' ').collect();
        if raw_chars.len() % 5 == 0 {
            self.input_state.push(' ');
            self.output_state.push(' ');
        }
    }

    fn build_rotor_string(rotor_positions: &[char]) -> String {
        rotor_positions.iter().map(|x| format!(" {}", *x)).collect()
    }
}
