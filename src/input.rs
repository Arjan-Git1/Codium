use std::io::{self, Write, stdout};

use crossterm::{
    cursor::{self, MoveLeft, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
};

use crate::{keys, mode::Mode};
pub fn input(mode: &mut Mode) -> io::Result<()> {
    let mut buffer = String::new();

    loop {
        if let Event::Key(event) = event::read()? {
            if event.kind == KeyEventKind::Press {
                match (event.modifiers, event.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                        *mode = Mode::Normal;
                        execute!(stdout(), cursor::SavePosition);
                        break;
                    }

                    (_, KeyCode::Char(c)) => {
                        keys::characters(&mut buffer, c)?;
                    }
                    (_, KeyCode::Enter) => {
                        keys::enter(&mut buffer)?;
                    }
                    (_, KeyCode::Backspace) => {
                        keys::backspace(&mut buffer)?;
                    }
                    (_, KeyCode::Up) => {
                        keys::up();
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
