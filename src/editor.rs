// src/editor.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

use std::io::{self, stdout, Write};

use enigma_core::reflectors::Reflector;
use enigma_core::rotors::{RotorEncode};
use enigma_core::{ArmyEnigma, Enigma, plugboard};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor<'a, A, B, C, D> {
    machine: &'a mut ArmyEnigma<A, B, C, D, plugboard::Plugboard>,
    should_quit: bool,
}

impl<'a, A: RotorEncode, B: RotorEncode, C: RotorEncode, D: Reflector> Editor<'a, A, B, C, D> {
    pub fn new(machine: &'a mut ArmyEnigma<A, B, C, D, plugboard::Plugboard>) -> Self {
        Self {
            machine: machine,
            should_quit: false,
        }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        while !self.should_quit {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;

        match pressed_key {
            Key::Char(c) => {
                if let Ok(o) = self.machine.keypress(c) {
                    print!("{}", o);
                    //return io::stdout().flush();
                }
            },
            Key::Ctrl('c') => self.should_quit = true,
            _ => {},
        }

        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!(e);
}

