pub mod normal;
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


