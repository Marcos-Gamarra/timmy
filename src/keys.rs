use crate::buffer::Buffer;

pub fn enter(buffer: &mut Buffer) {
    buffer.insert_empty_line();
}

pub fn left(buffer: &mut Buffer) {
    let cursor = buffer.cursor();
    let (cursor_x, cursor_y) = cursor.position();
    if cursor_x == 0 {
        return;
    }
    buffer.change_cursor_position(cursor_x - 1, cursor_y);
}

pub fn right(buffer: &mut Buffer) {
    let current_line_len = buffer.current_line().len();
    let cursor = buffer.cursor();
    let (cursor_x, cursor_y) = cursor.position();
    if current_line_len == cursor_x {
        return;
    }
    buffer.change_cursor_position(cursor_x + 1, cursor_y);
}

pub fn up(buffer: &mut Buffer) {
    let cursor = buffer.cursor();
    let (cursor_x, cursor_y) = cursor.position();
    if cursor_y == 0 {
        return;
    }
    buffer.change_cursor_position(cursor_x, cursor_y - 1);
}

pub fn down(buffer: &mut Buffer) {
    let cursor = buffer.cursor();
    let (cursor_x, cursor_y) = cursor.position();
    if cursor_y == buffer.number_of_lines() - 1 {
        return;
    }
    buffer.change_cursor_position(cursor_x, cursor_y + 1);
}
