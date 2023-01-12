use crate::cursor::{Cursor, CursorShape};
use crate::modes::Mode;
use std::io::{stdout, Write};

pub struct Buffer {
    body: Vec<String>,
    current_line_number: usize,
    cursor: Cursor,
    mode: Mode,
    term_size: (usize, usize),
    on_screen_range: (usize, usize),
}

impl Buffer {
    pub fn new() -> Buffer {
        let (term_x, term_y) = termion::terminal_size().unwrap();
        Buffer {
            body: vec![String::new()],
            current_line_number: 0,
            cursor: Cursor::new(),
            mode: Mode::Normal,
            term_size: (term_x as usize, term_y as usize),
            on_screen_range: (0, term_y as usize - 1),
        }
    }

    pub fn insert_empty_line(&mut self) {
        self.body[self.current_line_number].push_str("\r\n");
        self.current_line_number += 1;
        self.body.insert(self.current_line_number, String::new());
        self.cursor.change_position(0, self.current_line_number);
    }

    pub fn insert_char(&mut self, c: char) {
        let line = &mut self.body[self.current_line_number];
        let (cursor_x, cursor_y) = self.cursor.position();
        line.insert(cursor_x, c);
        self.cursor.change_position(cursor_x + 1, cursor_y);
    }

    pub fn remove_char(&mut self) {
        let (cursor_x, cursor_y) = self.cursor.position();
        if cursor_x == 0 {
            return;
        }
        let line = &mut self.body[self.current_line_number];
        line.remove(cursor_x - 1);
        self.cursor.change_position(cursor_x - 1, cursor_y);
    }

    pub fn is_first_line(&self) -> bool {
        if self.current_line_number == 0 {
            return true;
        }

        false
    }

    pub fn is_last_line(&self) -> bool {
        if self.current_line_number == self.body.len() - 1 {
            return true;
        }

        false
    }

    pub fn change_cursor_position(&mut self, x: usize, y: usize) {
        let (_, diff_y) = self.cursor.change_position(x, y);

        if diff_y < 0 {
            let diff = diff_y.abs() as usize;
            self.current_line_number -= diff;
        }

        if diff_y > 0 {
            let diff = diff_y as usize;
            self.current_line_number += diff;
        }
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn current_line(&self) -> &String {
        &self.body[self.current_line_number]
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn change_on_screen_range(&mut self, delta: isize) {
        self.current_line_number = (self.current_line_number as isize + delta) as usize;
        self.on_screen_range = (
            (self.on_screen_range.0 as isize + delta) as usize,
            (self.on_screen_range.1 as isize + delta) as usize,
        );
         
    }

    pub fn term_size(&self) -> (usize, usize) {
        self.term_size
    }

    pub fn change_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Normal => {
                self.cursor.change_shape(CursorShape::Block);
            }
            Mode::Insert => {
                self.cursor.change_shape(CursorShape::Bar);
            }

            _ => {}
        }
        self.mode = mode;
    }

    pub fn read_file(&mut self, file_name: &str) {
        let content = std::fs::read_to_string(file_name).unwrap();
        self.body = content
            .lines()
            .map(|line| {
                let mut line = line.to_string();
                line.push_str("\r\n");
                line
            })
            .collect();
    }

    pub fn render(&self) {
        write!(
            stdout(),
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        )
        .unwrap();

        for i in self.on_screen_range.0..self.on_screen_range.1 {
            print!("{}", self.body[i]);
        }
        self.cursor.render();
        stdout().flush().unwrap();
    }
}
