use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor { x: 0, y: 0 }
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn render(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(self.x as u16 + 1, self.y as u16 + 1)
        )
        .unwrap();
    }
}
