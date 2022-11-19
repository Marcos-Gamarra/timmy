use crate::input::Input;
use crate::modes::{switch_modes, Mode};
use crate::rendering::Render;

use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Buffer {
    stdout: RawTerminal<std::io::Stdout>,
    content: Vec<String>,
    current_line_number: usize,
    current_mode: Mode,
    cursor_position: (u16, u16),
    screen_size: (u16, u16),
    left_offset: u16,
}

impl Buffer {
    pub fn new(
        content: Vec<String>,
        current_line_number: usize,
        current_mode: Mode,
        cursor_position: (u16, u16),
        screen_size: (u16, u16),
        left_offset: u16,
    ) -> Self {
        let stdout = std::io::stdout().into_raw_mode().unwrap();
        Buffer {
            stdout,
            content,
            current_line_number,
            current_mode,
            cursor_position,
            screen_size,
            left_offset,
        }
    }

    pub fn stdout_mut(&mut self) -> &mut RawTerminal<std::io::Stdout> {
        &mut self.stdout
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn cursor_position(&self) -> (u16, u16) {
        self.cursor_position
    }

    pub fn set_cursor_position(&mut self, x: u16, y: u16) {
        self.cursor_position.0 = x;
        self.cursor_position.1 = y;
    }

    pub fn current_mode(&self) -> &Mode {
        &self.current_mode
    }

    pub fn set_current_mode(&mut self, new_mode: Mode) {
        self.current_mode = new_mode;
    }

    pub fn buffer(&self) -> &Vec<String> {
        &self.content
    }

    pub fn insert_line(&mut self, line_number: usize, line: String) {
        self.content.insert(line_number - 1, line);
    }

    pub fn current_line_mut(&mut self) -> &mut String {
        &mut self.content[self.current_line_number - 1]
    }

    pub fn current_line_number(&self) -> usize {
        self.current_line_number
    }

    pub fn set_current_line_number(&mut self, line_number: usize) {
        self.current_line_number = line_number;
    }

    pub fn current_line_len(&self) -> usize {
        self.content[self.current_line_number - 1].len()
    }

    pub fn is_current_line_empty(&self) -> bool {
        if self.current_line_len() == 1 {
            return true;
        }
        false
    }

    pub fn total_number_of_lines(&self) -> usize {
        self.content.len()
    }

    pub fn left_offest(&self) -> u16 {
        self.left_offset
    }

    pub fn handle_insert_mode(&mut self) {
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\t') => self.tab(4),
                Key::Char('\n') => self.enter(),
                Key::Esc => {
                    switch_modes(self, Mode::Normal);
                    break;
                }
                Key::Left => self.left(),
                Key::Right => self.right(),
                Key::Up => self.up(),
                Key::Down => self.down(),
                Key::Backspace => self.backspace(),
                Key::Char(c) => {
                    self.insert_character(c);
                }
                _ => {}
            }
            self.render();
        }
    }

    pub fn handle_normal_mode(&mut self) {
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Left => self.left(),
                Key::Right => self.right(),
                Key::Up => self.up(),
                Key::Down => self.down(),

                Key::Char('i') => {
                    switch_modes(self, Mode::Insert);
                    break;
                }
                Key::Char(':') => {
                    switch_modes(self, Mode::Command);
                    break;
                }
                //Key::Char('s') => {
                //    motions::linewise_forward_jump(stdout, &buffer[*current_line_number])
                //}
                //Key::Char('_') => motions::goto_beggining_of_line(stdout),
                //Key::Char('=') => {
                //    motions::goto_end_of_line(stdout, buffer[*current_line_number].len())
                //}
                _ => {}
            }
            self.stdout.flush().unwrap();
        }
    }
}

impl Render for Buffer {
    fn render(&mut self) {
        let mut y = 1;
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        for line in self.content.iter_mut() {
            y += 1;
            write!(self.stdout, "{}{}", line, termion::cursor::Goto(1, y)).unwrap();
        }
        write!(
            self.stdout,
            "{}{}",
            termion::clear::AfterCursor,
            termion::cursor::Goto(self.cursor_position.0, self.cursor_position.1),
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}

impl Input for Buffer {
    fn insert_character(&mut self, character: char) {
        let (x, y) = self.cursor_position();
        let left_offset = self.left_offest();
        let current_line = self.current_line_mut();
        current_line.insert((x - left_offset - 1) as usize, character);

        self.set_cursor_position(x + 1, y);
    }

    //inserts a newline at the current cursor position
    //right_side holds the characters to the right of the newline character that is being inserted
    //the current line is set to hold the left side of the line
    //right_side is pushed to the buffer after the current line
    //the cursor is updated
    fn enter(&mut self) {
        let (x, y) = self.cursor_position();
        let current_line_number = self.current_line_number();

        if self.is_current_line_empty() {
            self.insert_line(current_line_number + 1, String::from("\n"));
            self.set_cursor_position(x, y + 1);
            self.set_current_line_number(current_line_number + 1);
            return;
        }

        let current_line = self.current_line_mut();

        let right_side = current_line[x as usize - 1..].to_string();
        *current_line = current_line[..x as usize - 1].to_string();
        current_line.push('\n');

        self.insert_line(current_line_number + 1, right_side);
        self.set_cursor_position(1, y + 1);
        self.set_current_line_number(current_line_number + 1);
    }

    fn up(&mut self) {
        if self.current_line_number > 1 {
            self.current_line_number -= 1;
        } else {
            return;
        }

        if self.cursor_position.1 > 1 {
            self.cursor_position.1 -= 1
        }

        let current_line_len = self.current_line_len() as u16;
        let (x, y) = self.cursor_position();
        if current_line_len < x {
            self.set_cursor_position(current_line_len, y);
        }
    }

    fn down(&mut self) {
        let total_number_of_lines = self.total_number_of_lines();
        let current_line_number = self.current_line_number();

        if current_line_number < total_number_of_lines {
            self.current_line_number += 1;
        } else {
            return;
        }

        if self.cursor_position.1 < self.screen_size.1 {
            self.cursor_position.1 += 1;
        }

        let current_line_len = self.current_line_len() as u16;
        let (x, y) = self.cursor_position();

        if current_line_len < x {
            self.set_cursor_position(current_line_len, y);
        }
    }

    fn left(&mut self) {
        if self.cursor_position.0 == 1 {
            return;
        }

        self.cursor_position.0 -= 1;
    }

    fn right(&mut self) {
        let (x, y) = self.cursor_position();
        let current_line_len = self.current_line_len();
        if x as usize >= current_line_len {
            return;
        }

        self.set_cursor_position(x + 1, y);
    }

    fn backspace(&mut self) {
        if self.cursor_position.0 == self.left_offset as u16 + 1 {
            return;
        }

        self.content[self.current_line_number - 1]
            .remove((self.cursor_position.0 - self.left_offset - 2) as usize);

        self.cursor_position.0 -= 1;
    }

    fn tab(&mut self, number_of_spaces: u16) {
        let (x, _y) = &mut self.cursor_position;
        let distance_to_next_multiple = number_of_spaces - (*x % 4) + 1;
        let current_line = &mut self.content[self.current_line_number - 1];
        for _ in 0..distance_to_next_multiple {
            current_line.push(' ');
            *x += 1;
        }
    }
}
