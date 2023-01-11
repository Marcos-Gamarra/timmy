use crate::cursor::{Cursor, CursorShape};
use crate::modes::Mode;
use std::io::{stdout, Write};

pub struct Buffer {
    body: Vec<String>,
    current_line_number: usize,
    cursor: Cursor,
    mode: Mode,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            body: vec![String::new()],
            current_line_number: 0,
            cursor: Cursor::new(),
            mode: Mode::Normal,
        }
    }

    pub fn insert_empty_line(&mut self) {
        self.body[self.current_line_number].push_str("\r\n");
        self.current_line_number += 1;
        self.body.insert(self.current_line_number, String::new());
        self.cursor.set_position(0, self.current_line_number);
    }

    pub fn insert_char(&mut self, c: char) {
        let line = &mut self.body[self.current_line_number];
        let (cursor_x, cursor_y) = self.cursor.get_position();
        line.insert(cursor_x, c);
        self.cursor.set_position(cursor_x + 1, cursor_y);
    }

    pub fn remove_char(&mut self) {
        let (cursor_x, cursor_y) = self.cursor.get_position();
        if cursor_x == 0 {
            return;
        }
        let line = &mut self.body[self.current_line_number];
        line.remove(cursor_x - 1);
        self.cursor.set_position(cursor_x - 1, cursor_y);
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn current_line(&self) -> &String {
        &self.body[self.current_line_number]
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn change_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Normal => {
                self.cursor.change_shape(CursorShape::Block);
            }
            Mode::Insert => {
                self.cursor.change_shape(CursorShape::Bar);
            }

            _ => {},
        }
        self.mode = mode;
    }
    

    pub fn render(&self) {
        write!(
            stdout(),
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            self.body.join("")
        )
        .unwrap();
        self.cursor.render();
        stdout().flush().unwrap();
    }
}
