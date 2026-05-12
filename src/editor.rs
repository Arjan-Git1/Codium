use crate::mode;
use crate::{io, mode::Mode, piecetable::PieceTable};

use crossterm::terminal::{self, Clear, EnterAlternateScreen, disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute};
use std::io::stdout;
#[derive(Debug)]
pub struct Editor {
    pub mode: Mode,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub document: PieceTable,
}
impl Editor {
    pub fn render(&mut self) -> io::Result<()> {
        execute!(stdout(), cursor::MoveTo(self.cursor_x, self.cursor_y))?;

        Ok(())
    }
}
