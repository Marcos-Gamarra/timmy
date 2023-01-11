use crate::buffer::Buffer;
use crate::command_prompt::{Command, CommandPrompt};
use crate::keys;
use termion::event::Key;
use termion::input::TermRead;

#[derive(Copy, Clone)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

pub fn insert_mode(buffer: &mut Buffer) {
    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Char('\n') => keys::enter(buffer),
            Key::Char(c) => buffer.insert_char(c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => {
                buffer.change_mode(Mode::Normal);
                return;
            }
            Key::Left => keys::left(buffer),
            Key::Right => keys::right(buffer),
            Key::Up => keys::up(buffer),
            Key::Down => keys::down(buffer),
            Key::Backspace => buffer.remove_char(),
            _ => {}
        }
        buffer.render();
    }
}

pub fn normal_mode(buffer: &mut Buffer) {
    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Char('i') => {
                buffer.change_mode(Mode::Insert);
                return;
            }
            Key::Char(':') => {
                buffer.change_mode(Mode::Command);
                return;
            }
            _ => {}
        }
        buffer.render();
    }
}

pub fn command_mode(buffer: &mut Buffer, command_prompt: &mut CommandPrompt, exit_flag: &mut bool) {
    command_prompt.render();
    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Esc => {
                buffer.change_mode(Mode::Normal);
                return;
            }

            Key::Backspace => command_prompt.remove_char(),

            Key::Char('\n') => {
                let command = command_prompt.parse_command();
                match command {
                    Some(command) => match command {
                        Command::Exit => {
                            *exit_flag = true;
                            return;
                        }
                    },
                    None => {}
                }
            }

            Key::Char(c) => command_prompt.insert_char(c),

            _ => {}
        }
        command_prompt.render();
    }
}
