use crate::input::input;
use crate::{io, mode::Mode};
use crate::{keys, mode};
use piecetable::PieceTable;

use crossterm::event::KeyCode;
use crossterm::terminal::{
    self, Clear, ClearType, EnterAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{cursor, execute};
use std::io::{Write, stdout};
#[derive(Debug)]
pub struct Editor {
    pub mode: Mode,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub document: PieceTable,
}
impl Editor {
    pub fn render(&mut self) -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        print!("{}", self.document.display_result().unwrap());
        stdout().flush()?;
        execute!(stdout(), cursor::MoveTo(self.cursor_x, self.cursor_y),)?;

        Ok(())
    }
    pub fn input(&mut self) {
        let code = input(&mut self.mode);
        match code {
            Ok(KeyCode::Up) => {
                self.cursor_y = keys::up(self.cursor_y);
            }
            Ok(KeyCode::Down) => {
                self.cursor_y = keys::down(self.cursor_y);
            }
            Ok(KeyCode::Right) => {
                self.cursor_x = keys::right(self.cursor_x);
            }
            Ok(KeyCode::Left) => {
                self.cursor_x = keys::left(self.cursor_x);
            }
            Ok(_) => {}
            Err(E) => {}
        }
    }
    pub fn cursor_offset(&mut self, text: &str, cursor_x: u16, cursor_y: u16) -> u16 {
        let lines: Vec<&str> = text.lines().collect();

        let mut offset: u16 = 0;

        for y in 0..cursor_y {
            offset += lines[y as usize].len() as u16;
            offset += 1;
        }

        offset + cursor_x
    }
    pub fn insert(&mut self) {
        let code = input(&mut self.mode);
        let index = self.cursor_offset(
            self.document.display_result().unwrap().as_str(),
            self.cursor_x,
            self.cursor_y,
        );
        let index_usize = index as usize;
        match code {
            Ok(KeyCode::Char(c)) => {
                self.document.insert(&c.to_string(), index_usize);
                self.cursor_x = self.cursor_x + 1
            }
            Ok(_) => {}
            Err(E) => {}
        }
    }
}
