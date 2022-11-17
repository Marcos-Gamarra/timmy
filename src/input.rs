use std::io::{Stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::raw::RawTerminal;

pub trait Input {
    fn insert_character(&mut self, character: char);
    fn tab(&mut self, _number_of_spaces: u16) {}
    fn enter(&mut self) {}
    fn backspace(&mut self) {}
    fn right(&mut self) {}
    fn left(&mut self) {}
    fn up(&mut self) {}
    fn down(&mut self) {}
}


pub fn right(stdout: &mut RawTerminal<Stdout>) {
    let (x, y) = stdout.cursor_pos().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(x + 1, y)).unwrap();
}

pub fn left(stdout: &mut RawTerminal<Stdout>) {
    let (x, y) = stdout.cursor_pos().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(x - 1, y)).unwrap();
}

pub fn up(stdout: &mut RawTerminal<Stdout>, line_number: &mut usize) {
    let (x, y) = stdout.cursor_pos().unwrap();
    if *line_number == 0 {
        return;
    }
    *line_number -= 1;
    write!(stdout, "{}", termion::cursor::Goto(x, y - 1)).unwrap();
}

pub fn down(stdout: &mut RawTerminal<Stdout>, buffer_len: usize, line_number: &mut usize) {
    let (x, y) = stdout.cursor_pos().unwrap();
    if *line_number == buffer_len - 1 {
        return;
    }
    *line_number += 1;
    write!(stdout, "{}", termion::cursor::Goto(x, y + 1)).unwrap();
}
