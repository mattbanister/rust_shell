use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;


mod shell;

fn main() {
    let stdin = stdin();
    let mut sh = shell::Shell::new();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                sh.handle_newline();
            },
            Key::Char(c)    => {
                sh.handle_char(c);
            },
            Key::Backspace => {
                sh.handle_backspace();
            },
            Key::Ctrl('c')  => break,
            _               => (),
        }
    }
}