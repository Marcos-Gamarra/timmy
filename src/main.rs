use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

mod buffer;
mod command_prompt;
mod cursor;
mod keys;
mod modes;

use buffer::Buffer;
use command_prompt::CommandPrompt;
use modes::Mode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut buffer = Buffer::new();
    let mut exit_flag = false;
    let mut command_prompt = CommandPrompt::new();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();
    buffer.read_file("test_file");
    loop {
        match buffer.mode() {
            Mode::Insert => modes::insert_mode(&mut buffer),
            Mode::Normal => modes::normal_mode(&mut buffer),
            Mode::Command => {
                modes::command_mode(&mut buffer, &mut command_prompt, &mut exit_flag);
            }
        }

        if exit_flag {
            break;
        }
    }
}
