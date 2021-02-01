// src/ui/tui.rs
//
// Copyright (c) 2021
// Jeff Nettleton
//
// Licensed under the MIT license (http://opensource.org/licenses/MIT). This
// file may not be copied, modified, or distributed except according to those
// terms.

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use crossterm::execute;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};
use tui::Terminal;

use crate::ui::generic::{ApplicationExitReason, UiAgent};
use crate::ui::state::MachineState;
use enigma_core::reflectors::Reflector;
use enigma_core::rotors::{RotorEncode};
use enigma_core::{ArmyEnigma, Enigma, plugboard};

pub struct Tui<'a, A, B, C, D> {
    machine: &'a mut ArmyEnigma<A, B, C, D, plugboard::Plugboard>,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl<'a, A: RotorEncode, B: RotorEncode, C: RotorEncode, D: Reflector> Tui<'a, A, B, C, D> {
    pub fn new(
        machine: &'a mut ArmyEnigma<A, B, C, D, plugboard::Plugboard>,
    ) -> Result<Self> {
        let mut stdout = io::stdout();

        execute!(stdout, event::EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).with_context(|| "Failed to initialize Crossterm backend")?;
        crossterm::terminal::enable_raw_mode()?;
        terminal.hide_cursor()?;

        Ok(
            Tui {
                machine: machine,
                terminal: terminal,
            }
        )
    }
}

impl<'a, A: RotorEncode, B: RotorEncode, C: RotorEncode, D: Reflector> UiAgent for Tui<'a, A, B, C, D> {
    fn start(mut self) -> Result<ApplicationExitReason> {
        let mut state = MachineState::new(&self.machine.settings());
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);


        thread::spawn(move || {
            let mut last_tick = Instant::now();

            loop {
                if event::poll(tick_rate - last_tick.elapsed()).unwrap() {
                    let event = event::read().unwrap();
                    if let Event::Key(key) = event {
                        if let Err(_) = tx.send(Interrupt::KeyPressed(key)) {
                            return;
                        }
                    } else if let Event::Mouse(mouse) = event {
                        if let Err(_) = tx.send(Interrupt::MouseEvent(mouse)) {
                            return;
                        }
                    }
                }
                if last_tick.elapsed() > tick_rate {
                    if let Err(_) = tx.send(Interrupt::IntervalElapsed) {
                        return;
                    }
                    last_tick = Instant::now();
                }
            }
        });

        self.terminal.clear().with_context(|| {
            "Failed to clear terminal during drawing state. Do you have modern term?"
        })?;

        loop {
            match rx.recv()? {
                Interrupt::KeyPressed(event) => match event.code {
                    // exit
                    KeyCode::Char('c') if event.modifiers == KeyModifiers::CONTROL => {
                        return Ok(ApplicationExitReason::UserExit);
                    },
                    // reset
                    KeyCode::Char('r') if event.modifiers == KeyModifiers::CONTROL => {
                        self.machine.reset();
                        state = MachineState::new(&self.machine.settings());
                    },
                    KeyCode::Char(c) => match c {
                        'A'..='Z' | 'a'..='z' => {
                            let i = match c.is_lowercase() {
                                true => c.to_ascii_uppercase(),
                                false => c,
                            };

                            let o = self.machine.keypress(i).unwrap();
                            state.input_state.push_str(&format!("{}", i));
                            state.output_state.push_str(&format!("{}", o));

                            let raw_chars: Vec<char> = state.input_state.chars().filter(|x| *x != ' ').collect();
                            if raw_chars.len() % 5 == 0 {
                                state.input_state.push_str(" ");
                                state.output_state.push_str(" ");
                            }

                            state.update_rotors(&self.machine.settings());
                        },
                        _ => {},
                    },
                    _ => {},
                },
                Interrupt::MouseEvent(event) => match event {
                    _ => {},
                }
                _ => {},
            }

            self.terminal.draw(|mut f| {
                draw_layout_and_subcomponents(&mut f, &state);
            })?
        }
    }
}

fn draw_layout_and_subcomponents<K: Backend>(
    f: &mut Frame<K>,
    state: &MachineState,
) {
    let total_size = f.size();

    if let [left_plane, right_plane] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(total_size)[..]
    {
        if let [input_plane, output_plane] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .split(right_plane)[..]
        {
            draw_text(f, "Machine setup".into(), Some(&state.machine_state), left_plane);
            draw_text(f, "Input".into(), Some(&state.input_state), input_plane);
            draw_text(f, "Output".into(), Some(&state.output_state), output_plane);
        } else {
            panic!("Failed to draw vertically-split right plane");
        }
    } else {
        panic!("Failed to draw horizontally-split primary plane");
    }
}

fn draw_text<K: Backend>(
    f: &mut Frame<K>,
    heading: String,
    text_to_write: Option<&String>,
    area: Rect,
) {
    let block = Block::default()
        .title(heading)
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        )
        .border_type(BorderType::Rounded)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        );
    let default_string = String::new();
    let text = vec![Spans::from(
        Span::styled(
            format!("\n {} ", text_to_write.unwrap_or(&default_string)),
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        ),
    )];
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

enum Interrupt {
    KeyPressed(KeyEvent),
    MouseEvent(MouseEvent),
    IntervalElapsed,
}
