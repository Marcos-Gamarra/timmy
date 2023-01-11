use std::io::{stdout, Write};

pub enum Command {
    Exit,
}

pub struct CommandPrompt {
    body: String,
    cursor: usize,
}

impl CommandPrompt {
    pub fn new() -> Self {
        CommandPrompt {
            body: String::new(),
            cursor: 0,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.body.insert(self.cursor, c);
        self.cursor += 1;
    }

    pub fn remove_char(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.body.remove(self.cursor - 1);
        self.cursor -= 1;
    }

    pub fn parse_command(&self) -> Option<Command> {
        match self.body.as_str() {
            "quit" => Some(Command::Exit),
            _ => None,
        }
    }

    pub fn render(&self) {
        //get terminal size
        let (_, y) = termion::terminal_size().unwrap();
        write!(
            stdout(),
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, y - 1),
            " > ",
            self.body,
        )
        .unwrap();

        stdout().flush().unwrap();
    }
}
