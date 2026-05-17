use crossterm::{
    cursor::{self, MoveToNextLine},
    execute,
    style::Print,
};
use piecetable::PieceTable;
use std::io::{self, Write, stdout};

pub fn characters(buffer: &mut String, c: char) -> io::Result<()> {
    buffer.push(c);
    write!(io::stdout(), "{}", c)?;
    stdout().flush()?;
    Ok(())
}
pub fn enter(mut cursor_x: u16, mut cursor_y: u16) -> (u16, u16) {
    cursor_y = cursor_y + 1;
    cursor_x = 1;
    (cursor_x, cursor_y)
}
pub fn backspace(text: &mut PieceTable, offset: usize) {
    text.delete_char(offset - 1);
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
