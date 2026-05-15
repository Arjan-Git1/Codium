use std::io::{self, stdout};

use crossterm::{
    cursor::{self},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
};

use crate::{keys, mode::Mode};
pub fn input(mode: &mut Mode) -> io::Result<KeyCode> {
    let mut buffer = String::new();

    loop {
        if let Event::Key(event) = event::read()? {
            if event.kind == KeyEventKind::Press {
                return Ok(event.code);
            }
        }
    }
}
