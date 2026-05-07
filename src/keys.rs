use std::io::{self, Write, stdout};

use crossterm::{
    cursor::{self, MoveLeft, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
};

pub fn characters(buffer: &mut String, c: char) -> io::Result<()> {
    buffer.push(c);
    write!(io::stdout(), "{}", c)?;
    stdout().flush()?;
    Ok(())
}
pub fn enter(buffer: &mut String) -> io::Result<()> {
    println!();
    execute!(stdout(), MoveToNextLine(2))?;
    buffer.clear();
    Ok(())
}
pub fn backspace(buffer: &mut String) -> io::Result<()> {
    if !buffer.is_empty() {
        execute!(
            stdout(),
            cursor::MoveLeft(1),
            Print(" "),
            cursor::MoveLeft(1)
        )?
    }
    Ok(())
}
