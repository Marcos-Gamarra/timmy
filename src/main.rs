use std::fs::File;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use std::io::Write;

use std::env;

mod jump;
mod keys;

fn main() {
    let mut file = handle_command_invocation(&mut env::args()).unwrap();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let stdin = std::io::stdin();
    let mut buffer: Vec<String> = vec![String::new()];
    let mut line_number: usize = 0;
    let mut mode = Mode::Normal;
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    stdout.flush().unwrap();
    take_input(&mut stdout, stdin, &mut buffer, &mut line_number, &mut mode);
    write_buffer_to_file(&buffer, &mut file);
}

fn take_input(
    stdout: &mut RawTerminal<std::io::Stdout>,
    stdin: std::io::Stdin,
    buffer: &mut Vec<String>,
    line_number: &mut usize,
    mode: &mut Mode,
) {
    for c in stdin.keys() {
        match mode {
            Mode::Insert => match c.unwrap() {
                Key::Ctrl('c') => {
                    break;
                }
                Key::Char('\t') => keys::tab(stdout, &mut buffer[*line_number], 4),
                Key::Char('\n') => keys::enter(stdout, buffer, line_number),
                Key::Esc => {
                    *mode = Mode::Normal;
                    write!(stdout, "{}", termion::cursor::SteadyBlock).unwrap();
                }
                Key::Char(c) => keys::insertion(stdout, &mut buffer[*line_number], c),
                Key::Left => keys::left(stdout),
                Key::Right => keys::right(stdout),
                Key::Up => keys::up(stdout, line_number),
                Key::Down => keys::down(stdout, buffer.len(), line_number),
                Key::Backspace => keys::backspace(stdout, buffer, *line_number),
                _ => {}
            },

            Mode::Normal => match c.unwrap() {
                Key::Char('i') => {
                    *mode = Mode::Insert;
                    write!(stdout, "{}", termion::cursor::SteadyBar).unwrap();
                }
                Key::Char('s') => jump::linewise_forward_jump(stdout, &buffer[*line_number]),
                _ => {}
            },
        }

        stdout.flush().unwrap();
    }
}

fn write_buffer_to_file(buffer: &Vec<String>, file: &mut File) {
    for line in buffer {
        write!(file, "{}", line).unwrap();
    }
}

fn handle_command_invocation(args: &mut std::env::Args) -> Result<std::fs::File, std::io::Error> {
    if let Some(mut first_arg) = args.nth(1) {
        first_arg = first_arg.to_string();
        println!("{}", first_arg);
        return File::create(first_arg);
    } else {
        return File::create("no_name.txt");
    }
}

enum Mode {
    Normal,
    Insert,
}
