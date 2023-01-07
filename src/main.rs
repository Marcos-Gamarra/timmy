use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod buffer;
mod cursor;
mod keys;

use buffer::Buffer;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut buffer = Buffer::new();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            //Key::Char(c) => print!("{}", c),
            Key::Char('\n') => {
                keys::enter(&mut buffer);
            }
            Key::Char(c) => {
                buffer.insert_char(c);
            }
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => {
                keys::left(&mut buffer);
            }
            Key::Right => {
                keys::right(&mut buffer);
            }
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => {
                buffer.remove_char();
            }
            _ => {}
        }
        buffer.render();
        stdout.flush().unwrap();
    }
}
