mod jump;
mod keys;
mod layout;
mod modes;

use layout::commandline::CommandLine;
use modes::Mode;

use std::fs::File;

use termion::raw::{IntoRawMode, RawTerminal};

use std::io::Write;

use std::env;

fn main() {
    let mut file = handle_command_invocation(&mut env::args()).unwrap();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut buffer: Vec<String> = vec![String::new()];
    let mut line_number: usize = 0;
    let mut current_mode = modes::Mode::Normal;
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    stdout.flush().unwrap();
    take_input(
        &mut stdout,
        &mut buffer,
        &mut line_number,
        &mut current_mode,
    );
    write_buffer_to_file(&buffer, &mut file);
}

fn take_input(
    stdout: &mut RawTerminal<std::io::Stdout>,
    buffer: &mut Vec<String>,
    line_number: &mut usize,
    current_mode: &mut Mode,
) {
    let mut command_line = CommandLine::new(String::new(), String::from(" > "));
    let mut is_running = true;
    while is_running {
        match current_mode {
            Mode::Insert => modes::handle_insert_mode(stdout, buffer, line_number, current_mode),
            Mode::Normal => modes::handle_normal_mode(stdout, buffer, line_number, current_mode),
            Mode::Command => {
                is_running = modes::handle_command_mode(stdout, &mut command_line);
            }
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
