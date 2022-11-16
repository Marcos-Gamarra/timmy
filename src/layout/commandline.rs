use crate::input;
use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{RawTerminal, IntoRawMode};

pub struct CommandLine {
    stdout: RawTerminal<std::io::Stdout>,
    value: String,
    prompt: String,
}

impl CommandLine {
    pub fn new(value: String, prompt: String) -> Self {
        let stdout = std::io::stdout().into_raw_mode().unwrap();
        CommandLine {
            stdout,
            value,
            prompt,
        }
    }

    fn write_prompt(&mut self) {
        let (_, term_size_y) = termion::terminal_size().unwrap();
        write!(
            self.stdout,
            "{}{}{}",
            termion::cursor::Goto(1, term_size_y),
            self.prompt,
            termion::clear::AfterCursor,
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }


    pub fn handle_command_mode(&mut self) -> bool {
        self.write_prompt();
        self.take_input();
        let command = &self.value;
        if command == "exit" {
            self.value.clear();
            return false;
        }
        self.value.clear();
        true
    }

    fn take_input(&mut self) {
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    break;
                }
                Key::Backspace => input::backspace(&mut self.stdout, &mut self.value, self.prompt.len()),
                Key::Char(c) => {
                    input::insert_character(&mut self.stdout, &mut self.value, c, self.prompt.len())
                }
                _ => {}
            }
            self.stdout.flush().unwrap();
        }
    }
}
