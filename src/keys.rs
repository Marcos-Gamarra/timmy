use std::io::{Stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::raw::RawTerminal;

//writes the character under the current cursor position
pub fn insertion(stdout: &mut RawTerminal<Stdout>, current_line: &mut String, character: char) {
    let (x, y) = stdout.cursor_pos().unwrap();

    current_line.insert(x as usize - 1, character);

    write!(
        stdout,
        "{}{}{}{}",
        termion::clear::CurrentLine,
        termion::cursor::Goto(1, y),
        current_line,
        termion::cursor::Goto(x + 1, y),
    )
    .unwrap();
}

//handles enter key:
//pushes '\n' to current line, increase line_number by one,
//inserts new empty line into buffer and moves the cursor
//to the left most of the new line
pub fn enter(stdout: &mut RawTerminal<Stdout>, buffer: &mut Vec<String>, line_number: &mut usize) {
    let (x, _y) = stdout.cursor_pos().unwrap();
    let x = x as usize - 1;
    buffer[*line_number].insert(x, '\n');
    *line_number += 1;
    buffer.insert(*line_number, String::new());
    write!(
        stdout,
        "{}",
        termion::cursor::Goto(1, *line_number as u16 + 1)
    )
    .unwrap();
}

pub fn backspace(stdout: &mut RawTerminal<Stdout>, buffer: &mut Vec<String>, line_number: usize) {
    let (x, y) = stdout.cursor_pos().unwrap();

    if x == 1 {
        return;
    }

    buffer[line_number].remove(x as usize - 2);

    write!(
        stdout,
        "{}{}{}{}",
        termion::clear::CurrentLine,
        termion::cursor::Goto(1, y),
        buffer[line_number],
        termion::cursor::Goto(x - 1, y)
    )
    .unwrap();
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

pub fn tab(stdout: &mut RawTerminal<Stdout>, current_line: &mut String, number_of_spaces: u16) {
    let (x, _y) = stdout.cursor_pos().unwrap();
    let distance_to_next_multiple = number_of_spaces - (x % 4) + 1;
    for _ in 0..distance_to_next_multiple {
        insertion(stdout, current_line, ' ');
    }
}
