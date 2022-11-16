use crate::keys;
use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;

pub struct CommandLine {
    value: String,
    prompt: String,
}

impl CommandLine {
    pub fn new(value: String, prompt: String) -> Self {
        CommandLine { value, prompt }
    }

    pub fn get_value(&mut self) -> &String {
        let value = &self.value;
        value
    }

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    fn write_prompt(&self, stdout: &mut RawTerminal<std::io::Stdout>) {
        let (_, term_size_y) = termion::terminal_size().unwrap();
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, term_size_y),
            self.prompt,
            termion::clear::AfterCursor,
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn render(&mut self, stdout: &mut RawTerminal<std::io::Stdout>) {
        self.write_prompt(stdout);
        self.take_input(stdout);
    }

    fn take_input(&mut self, stdout: &mut RawTerminal<std::io::Stdout>) {
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    break;
                }
                Key::Backspace => keys::backspace(stdout, &mut self.value, self.prompt.len()),
                Key::Char(c) => {
                    keys::insert_character(stdout, &mut self.value, c, self.prompt.len())
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}
