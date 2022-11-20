use super::CommandLine;
use std::io::Write;

impl CommandLine {
    pub fn render(&mut self) {
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::cursor::Goto(1, self.cursor_position.1),
            self.prompt,
            self.value,
            termion::clear::AfterCursor,
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}
