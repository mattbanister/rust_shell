use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                writeln!(stdout, "New line entered\r").unwrap();
                stdout.flush().unwrap();
            },
            Key::Char(c)    => {
                writeln!(stdout, "char {} entered\r", c).unwrap();
                stdout.flush().unwrap();
            },
            Key::Backspace => {
                writeln!(stdout, "backspace entered\r").unwrap();
                stdout.flush().unwrap();
            },
            Key::Ctrl('c')  => break,
            _               => (),
        }
    }
}