use std::io::{self, stdout};

use crossterm::{
    cursor::{self},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
};

use crate::mode;
use mode::Mode;

pub fn commands(mode: &mut Mode) -> io::Result<()> {
    loop {
        if let Event::Key(event) = event::read()? {
            if event.kind == KeyEventKind::Press {
                match (event.modifiers, event.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                        *mode = Mode::Quit;
                        break;
                    }
                    (_, KeyCode::Char('I')) => {
                        *mode = Mode::Editing;
                        execute!(stdout(), cursor::RestorePosition)?;
                        break;
                    }

                    _ => {}
                }
            }
        }
    }
    Ok(())
}
