use crate::Mode;

use termion::event::Key;
use termion::input::TermRead;
use std::io::{Stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::raw::RawTerminal;

pub fn enter_insert_mode(stdout: &mut RawTerminal<Stdout>, current_mode: &mut Mode, forward: bool) {
    let (mut x, y) = stdout.cursor_pos().unwrap();
    if forward {
        x += 1;
    }
    write!(stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
    *current_mode = Mode::Insert;
}

pub fn goto_end_of_line(stdout: &mut RawTerminal<Stdout>, current_line_len: usize) {
    let (_x, y) = stdout.cursor_pos().unwrap();
    write!(
        stdout,
        "{}",
        termion::cursor::Goto(current_line_len as u16, y)
    )
    .unwrap();
}

pub fn goto_beggining_of_line(stdout: &mut RawTerminal<Stdout>) {
    let (_x, y) = stdout.cursor_pos().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, y)).unwrap();
}

pub fn linewise_forward_jump(stdout: &mut RawTerminal<Stdout>, current_line: &String) {
    let mut target = String::new();
    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Esc => {
                break;
            }
            Key::Char(c) => {
                target.push(c);
                if target.len() == 2 {
                    break;
                }
            }

            _ => {}
        }
    }

    let (_x, y) = stdout.cursor_pos().unwrap();
    let target_index = current_line.find(&target);

    if let Some(target_index) = target_index {
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(target_index as u16 + 1, y)
        )
        .unwrap()
    }
}
