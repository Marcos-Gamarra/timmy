use crate::buffer::Buffer;
use std::io::Write;

pub enum Mode {
    Normal,
    Insert,
    Command,
}

pub fn switch_modes(buffer: &mut Buffer, new_mode: Mode) {
    match new_mode {
        Mode::Normal => {
            buffer.set_current_mode(Mode::Normal);
            write!(buffer.stdout_mut(), "{}", termion::cursor::SteadyBlock).unwrap();
        }
        Mode::Insert => {
            buffer.set_current_mode(Mode::Insert);
            write!(buffer.stdout_mut(), "{}", termion::cursor::SteadyBar).unwrap();
        }
        Mode::Command => {
            let (_, term_size_y) = termion::terminal_size().unwrap();
            write!(
                buffer.stdout_mut(),
                "{}{}:",
                termion::cursor::Goto(1, term_size_y),
                termion::clear::CurrentLine,
            )
            .unwrap();
            buffer.set_current_mode(Mode::Command);
        }
    }
}
