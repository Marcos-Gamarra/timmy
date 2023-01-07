use crate::buffer::Buffer;

pub fn enter(buffer: &mut Buffer) {
    buffer.insert_empty_line();
}

pub fn left(buffer: &mut Buffer) {
    let cursor = buffer.cursor_mut();
    let (cursor_x, cursor_y) = cursor.get_position();
    if cursor_x == 0 {
        return;
    }
    cursor.set_position(cursor_x - 1, cursor_y);
}

pub fn right(buffer: &mut Buffer) {
    let current_line_len = buffer.current_line().len();
    let cursor = buffer.cursor_mut();
    let (cursor_x, cursor_y) = cursor.get_position();
    if current_line_len == cursor_x {
        return;
    }
    cursor.set_position(cursor_x + 1, cursor_y);
}
