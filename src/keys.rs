use std::io::{self, Write, stdout};

use crossterm::{
    cursor::{self, MoveToNextLine},
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
    execute!(stdout(), MoveToNextLine(1))?;
    buffer.clear();
    Ok(())
}
pub fn backspace(buffer: &mut String) -> io::Result<()> {
    if !buffer.is_empty() {
        buffer.pop();
        execute!(
            stdout(),
            cursor::MoveLeft(1),
            Print(" "),
            cursor::MoveLeft(1)
        )?
    }
    if _cursor_position() {
        execute!(stdout(), cursor::MoveToPreviousLine(01))?
    }

    Ok(())
}
pub fn _cursor_position() -> bool {
    if let Ok((column, _row)) = cursor::position() {
        return column == 0;
    } else {
        return false;
    }
}
pub fn up() -> io::Result<()> {
    execute!(stdout(), cursor::MoveUp(1))?;

    Ok(())
}
pub fn down() -> io::Result<()> {
    execute!(stdout(), cursor::MoveDown(1))?;
    Ok(())
}
pub fn right() -> io::Result<()> {
    execute!(stdout(), cursor::MoveRight(1))?;
    Ok(())
}
pub fn left() -> io::Result<()> {
    execute!(stdout(), cursor::MoveLeft(1))?;
    Ok(())
}
