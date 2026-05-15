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
pub fn up(mut y: u16) -> u16 {
    y = y - 1;

    return y;
}
pub fn down(mut y: u16) -> u16 {
    y = y + 1;
    return y;
}
pub fn right(mut x: u16) -> u16 {
    x = x + 1;
    return x;
}
pub fn left(mut x: u16) -> u16 {
    x = x - 1;
    return x;
}
