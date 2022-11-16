mod input;
mod layout;
mod modes;

use layout::{commandline::CommandLine, content::Content};
use modes::Mode;

use termion::raw::IntoRawMode;

use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = handle_command_invocation(&mut env::args()).unwrap();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let buffer: Vec<String> = vec![String::new()];
    let mut commandline = CommandLine::new(String::new(), String::from(" > "));
    let mut content = Content::new(buffer, 0, Mode::Normal, (0, 0), 0);
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    stdout.flush().unwrap();
    take_input(&mut content, &mut commandline);
    write_buffer_to_file(content.get_buffer(), &mut file);
}

fn take_input(content: &mut Content, commandline: &mut CommandLine) {
    let mut is_running = true;
    while is_running {
        match *content.get_current_mode() {
            Mode::Insert => content.handle_insert_mode(),
            Mode::Normal => content.handle_normal_mode(),
            Mode::Command => {
                is_running = commandline.handle_command_mode();
            }
        }
        content.flush();
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
