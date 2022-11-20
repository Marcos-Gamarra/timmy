use super::Buffer;

impl Buffer {
    //insert the character at the position where cursor is at that moment, 
    //then, it moves the cursor one position to the right
    pub fn insert_character(&mut self, character: char) {
        let (x, y) = self.cursor_position();
        let left_offset = self.left_offest();
        let current_line = self.current_line_mut();
        current_line.insert((x - left_offset - 1) as usize, character);

        self.set_cursor_position(x + 1, y);
    }

    //Inserts a newline at the current cursor position.
    //if the current line is empty:
    //it inserts a new line below the current line, increases the 
    //current_line_number by one and moves the cursor down

    //if the line is not empty:
    //it uses right_side to hold the part of the line that is after the cursor
    //the current_line is updated to hold the part of the line before the cursor
    //then, the right_side is inserted into the buffer as a new line
    //finally, the position of the cursor and the number of the current line are updated
    pub fn enter(&mut self) {
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

     
    pub fn up(&mut self) {
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

    pub fn down(&mut self) {
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

    pub fn left(&mut self) {
        if self.cursor_position.0 == 1 {
            return;
        }

        self.cursor_position.0 -= 1;
    }

    pub fn right(&mut self) {
        let (x, y) = self.cursor_position();
        let current_line_len = self.current_line_len();
        if x as usize >= current_line_len {
            return;
        }

        self.set_cursor_position(x + 1, y);
    }

    pub fn backspace(&mut self) {
        if self.cursor_position.0 == self.left_offset as u16 + 1 {
            return;
        }

        self.content[self.current_line_number - 1]
            .remove((self.cursor_position.0 - self.left_offset - 2) as usize);

        self.cursor_position.0 -= 1;
    }

    pub fn tab(&mut self, number_of_spaces: u16) {
        let (x, _y) = &mut self.cursor_position;
        let distance_to_next_multiple = number_of_spaces - (*x % 4) + 1;
        let current_line = &mut self.content[self.current_line_number - 1];
        for _ in 0..distance_to_next_multiple {
            current_line.push(' ');
            *x += 1;
        }
    }
}
