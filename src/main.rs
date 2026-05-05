use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> io::Result<()> {
    enable_raw_mode().expect("Error");
    loop {
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char(c) => {
                    println!("{}", c);
                }
                _ => {}
            }
        }
    }

    disable_raw_mode().expect("Error");
    Ok(())
}
