use super::CommandLine;

impl CommandLine {
    pub fn insert_character(&mut self, character: char) {
        self.value.insert(
            self.cursor_position.0 as usize - self.prompt.len() - 1,
            character,
        );

        self.cursor_position.0 += 1;
    }

    pub fn backspace(&mut self) {
        if self.value.len() == 0 {
            return;
        }

        self.value
            .remove(self.cursor_position.0 as usize - self.prompt.len() - 2);
        self.cursor_position.0 -= 1;
    }
}
