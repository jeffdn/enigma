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
    pub fn new(rotor_positions: &Vec<char>) -> Self {
        Self {
            machine_state: rotor_positions.iter().map(|x| format!(" {}", *x)).collect(),
            input_state: "".into(),
            output_state: "".into(),
        }
    }

    pub fn update_rotors(&mut self, rotor_positions: &Vec<char>) {
        self.machine_state = rotor_positions.iter().map(|x| format!(" {}", *x)).collect();
    }
}
