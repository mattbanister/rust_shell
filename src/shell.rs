use termion::raw::IntoRawMode;
use std::io::{Stdout, Write, stdout};

pub struct Shell {
    stdout: termion::raw::RawTerminal<Stdout>
}

impl Shell {
    pub fn new() -> Self {
        let stdout = stdout().into_raw_mode().unwrap();
        Shell {
            stdout: termion::raw::RawTerminal::from(stdout)
        }
    }

    pub fn handle_char(&mut self, c: char) {
        writeln!(self.stdout, "char {} entered\r", c).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn handle_newline(&mut self) {
        writeln!(self.stdout, "New line entered\r").unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn handle_backspace(&mut self) {
        writeln!(self.stdout, "backspace entered\r").unwrap();
        self.stdout.flush().unwrap();
    }
}
