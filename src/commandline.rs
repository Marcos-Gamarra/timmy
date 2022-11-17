use crate::input::Input;
use crate::rendering::Render;
use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct CommandLine {
    stdout: RawTerminal<std::io::Stdout>,
    value: String,
    prompt: String,
    cursor_position: (u16, u16),
}

impl CommandLine {
    pub fn new(value: String, prompt: String, cursor_position: (u16, u16)) -> Self {
        let stdout = std::io::stdout().into_raw_mode().unwrap();
        let cursor_position = (cursor_position.0 + prompt.len() as u16, cursor_position.1);
        CommandLine {
            stdout,
            value,
            prompt,
            cursor_position,
        }
    }

    pub fn handle_command_mode(&mut self) -> bool {
        self.render();
        self.take_input();
        let command = &self.value;
        if command == "exit" {
            self.value.clear();
            return false;
        }

        self.cursor_position.0 = self.prompt.len() as u16 + 1;
        self.value.clear();
        true
    }

    fn take_input(&mut self) {
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => break,
                Key::Backspace => self.backspace(),
                Key::Char(c) => self.insert_character(c),
                _ => {}
            }
            self.render();
        }
    }
}

impl Render for CommandLine {
    fn render(&mut self) {
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::cursor::Goto(1, self.cursor_position.1),
            self.prompt,
            self.value,
            termion::clear::AfterCursor,
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}

impl Input for CommandLine {
    fn insert_character(&mut self, character: char) {
        self.value.insert(
            self.cursor_position.0 as usize - self.prompt.len() - 1,
            character,
        );

        self.cursor_position.0 += 1;
    }

    fn backspace(&mut self) {
        if self.value.len() == 0 {
            return;
        }

        self.value
            .remove(self.cursor_position.0 as usize - self.prompt.len() - 2);
        self.cursor_position.0 -= 1;
    }
}
