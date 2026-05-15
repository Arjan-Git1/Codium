use std::{
    io::{self},
    vec,
};
mod commands;
mod editor;
mod input;
mod keys;
mod mode;
use piecetable::PieceTable;

use crossterm::{
    ExecutableCommand, execute,
    terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use editor::Editor;
use io::stdout;

use crate::{input::input, mode::Mode};
fn main() -> io::Result<()> {
    let document = PieceTable::from_str("Hello World");
    let mut editor = Editor {
        mode: Mode::Normal,
        cursor_x: 0,
        cursor_y: 0,
        document: document,
    };
    enable_raw_mode();
    execute!(stdout(), EnterAlternateScreen)?;
    loop {
        editor.render()?;
        editor.input();
        editor.insert();
    }
    disable_raw_mode();
}
