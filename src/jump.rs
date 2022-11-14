use std::io::{Stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;

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
