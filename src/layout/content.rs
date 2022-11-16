use crate::input;
use crate::modes::normal::motions;
use crate::modes::{switch_modes, Mode};
use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Content {
    stdout: RawTerminal<std::io::Stdout>,
    buffer: Vec<String>,
    current_line_number: usize,
    current_mode: Mode,
    lines_on_screen: (usize, usize),
    left_offset: usize,
}

impl Content {
    pub fn new(
        buffer: Vec<String>,
        current_line_number: usize,
        current_mode: Mode,
        lines_on_screen: (usize, usize),
        left_offset: usize,
    ) -> Self {
        let stdout = std::io::stdout().into_raw_mode().unwrap();
        Content {
            stdout,
            buffer,
            current_line_number,
            current_mode,
            lines_on_screen,
            left_offset,
        }
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn get_current_mode(&self) -> &Mode {
        &self.current_mode
    }

    pub fn get_buffer(&self) -> &Vec<String> {
        &self.buffer
    }

    pub fn handle_insert_mode(&mut self) {
        let stdout = &mut self.stdout;
        let buffer = &mut self.buffer;
        let line_number = &mut self.current_line_number;
        let current_mode = &mut self.current_mode;
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\t') => input::tab(stdout, &mut buffer[*line_number], 4),
                Key::Char('\n') => input::enter(stdout, buffer, line_number),
                Key::Esc => {
                    switch_modes(stdout, current_mode, Mode::Normal);
                    break;
                }
                Key::Left => input::left(stdout),
                Key::Right => input::right(stdout),
                Key::Up => input::up(stdout, line_number),
                Key::Down => input::down(stdout, buffer.len(), line_number),
                Key::Backspace => input::backspace(stdout, &mut buffer[*line_number], 0),
                Key::Char(c) => input::insert_character(stdout, &mut buffer[*line_number], c, 0),
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }

    pub fn handle_normal_mode(&mut self) {
        let stdout = &mut self.stdout;
        let buffer = &mut self.buffer;
        let line_number = &mut self.current_line_number;
        let current_mode = &mut self.current_mode;
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Left => input::left(stdout),
                Key::Right => input::right(stdout),
                Key::Up => input::up(stdout, line_number),
                Key::Down => input::down(stdout, buffer.len(), line_number),
                Key::Char('i') => {
                    switch_modes(stdout, current_mode, Mode::Insert);
                    break;
                }
                Key::Char(':') => {
                    switch_modes(stdout, current_mode, Mode::Command);
                    break;
                }
                Key::Char('s') => motions::linewise_forward_jump(stdout, &buffer[*line_number]),
                Key::Char('_') => motions::goto_beggining_of_line(stdout),
                Key::Char('=') => motions::goto_end_of_line(stdout, buffer[*line_number].len()),
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}
