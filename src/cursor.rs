use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

pub enum CursorShape {
    Block,
    Bar,
}

pub struct Cursor {
    x: usize,
    y: usize,
    shape: CursorShape,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            x: 0,
            y: 0,
            shape: CursorShape::Block,
        }
    }

    //changes the cursor position to the given x and y, and returns
    //the difference between the new position and the old position
    pub fn change_position(&mut self, x: usize, y: usize) -> (i32, i32) {
        let (diff_x, diff_y) = (x as i32 - self.x as i32, y as i32 - self.y as i32);
        self.x = x;
        self.y = y;

        (diff_x, diff_y)
    }

    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn change_shape(&mut self, shape: CursorShape) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        match shape {
            CursorShape::Block => {
                write!(stdout, "{}", termion::cursor::SteadyBlock,).unwrap();
            }

            CursorShape::Bar => {
                write!(stdout, "{}", termion::cursor::SteadyBar,).unwrap();
            }
        }

        self.shape = shape;
        stdout.flush().unwrap();
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
