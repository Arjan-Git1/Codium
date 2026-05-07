use std::io::{self, Write, stdout};

use crossterm::{
    cursor::{self, MoveLeft, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
};

use crate::keys;
pub fn input() -> io::Result<()> {
    let mut buffer = String::new();

    loop {
        if let Event::Key(event) = event::read()? {
            if event.kind == KeyEventKind::Press {
                match (event.modifiers, event.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                        break;
                    }

                    (_, KeyCode::Char(c)) => {
                        keys::characters(&mut buffer, c)?;
                    }
                    (_, KeyCode::Enter) => {
                        println!();
                        execute!(stdout(), MoveToNextLine(2))?;
                        buffer.clear();
                    }
                    (_, KeyCode::Backspace) => {
                        if !buffer.is_empty() {
                            execute!(
                                stdout(),
                                cursor::MoveLeft(1),
                                Print(" "),
                                cursor::MoveLeft(1)
                            )?
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
