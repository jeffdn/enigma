// src/lib.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

mod editor;

use editor::Editor;

use std::collections::HashMap;

use enigma_core::plugboard::Plugboard;
use enigma_core::reflectors;
use enigma_core::rotors::{self, RotorEncode};
use enigma_core::{ArmyEnigma, plugboard};

fn main() {
    let mut machine = ArmyEnigma::new(
        rotors::RotorIII::new('G', 'E'),
        rotors::RotorII::new('E', 'H'),
        rotors::RotorIV::new('W', 'R'),
        reflectors::ReflectorC{},
        plugboard! {
            'E' => 'R',
            'S' => 'A',
            'T' => 'Z'
        },
    );
    let mut editor = Editor::new(&mut machine);

    editor.run();
}
