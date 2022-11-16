pub mod normal;

use crate::jump;
use crate::keys;
use crate::layout::commandline::CommandLine;
use normal::motions;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;

use std::io::Write;

pub enum Mode {
    Normal,
    Insert,
    Command,
}

pub fn switch_modes(
    stdout: &mut RawTerminal<std::io::Stdout>,
    current_mode: &mut Mode,
    new_mode: Mode,
) {
    match new_mode {
        Mode::Normal => {
            *current_mode = Mode::Normal;
            write!(stdout, "{}", termion::cursor::SteadyBlock).unwrap();
        }
        Mode::Insert => {
            *current_mode = Mode::Insert;
            write!(stdout, "{}", termion::cursor::SteadyBar).unwrap();
        }
        Mode::Command => {
            let (_, term_size_y) = termion::terminal_size().unwrap();
            write!(
                stdout,
                "{}{}:",
                termion::cursor::Goto(1, term_size_y),
                termion::clear::CurrentLine,
            )
            .unwrap();
            *current_mode = Mode::Command;
        }
    }
}

pub fn handle_normal_mode(
    stdout: &mut RawTerminal<std::io::Stdout>,
    buffer: &mut Vec<String>,
    line_number: &mut usize,
    current_mode: &mut Mode,
) {
    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Left => keys::left(stdout),
            Key::Right => keys::right(stdout),
            Key::Up => keys::up(stdout, line_number),
            Key::Down => keys::down(stdout, buffer.len(), line_number),
            Key::Char('i') => {
                switch_modes(stdout, current_mode, Mode::Insert);
                break;
            }
            Key::Char(':') => {
                switch_modes(stdout, current_mode, Mode::Command);
                break;
            }
            Key::Char('s') => jump::linewise_forward_jump(stdout, &buffer[*line_number]),
            Key::Char('_') => motions::goto_beggining_of_line(stdout),
            Key::Char('=') => motions::goto_end_of_line(stdout, buffer[*line_number].len()),
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

pub fn handle_insert_mode(
    stdout: &mut RawTerminal<std::io::Stdout>,
    buffer: &mut Vec<String>,
    line_number: &mut usize,
    current_mode: &mut Mode,
) {
    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Char('\t') => keys::tab(stdout, &mut buffer[*line_number], 4),
            Key::Char('\n') => keys::enter(stdout, buffer, line_number),
            Key::Esc => {
                switch_modes(stdout, current_mode, Mode::Normal);
                break;
            }
            Key::Left => keys::left(stdout),
            Key::Right => keys::right(stdout),
            Key::Up => keys::up(stdout, line_number),
            Key::Down => keys::down(stdout, buffer.len(), line_number),
            Key::Backspace => keys::backspace(stdout, &mut buffer[*line_number], 0),
            Key::Char(c) => keys::insert_character(stdout, &mut buffer[*line_number], c, 0),
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

pub fn handle_command_mode(
    stdout: &mut RawTerminal<std::io::Stdout>,
    command_line: &mut CommandLine,
) -> bool {
    command_line.render(stdout);
    let command = command_line.get_value();
    if command == "exit" {
        command_line.clear_value();
        return false;
    }
    command_line.clear_value();
    true
}
