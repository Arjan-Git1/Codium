use std::{
    io::{self},
    vec,
};
mod commands;
mod editor;
mod input;
mod keys;
mod mode;
mod piecetable;

use crossterm::{
    ExecutableCommand, execute,
    terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use editor::Editor;
use io::stdout;

use crate::{mode::Mode, piecetable::Piece, piecetable::PieceTable};
fn main() -> io::Result<()> {
    let piecetable = PieceTable {
        original: String::new(),
        add: String::new(),
        pieces: Vec::new(),
    };
    let mut editor = Editor {
        mode: Mode::Normal,
        cursor_x: 0,
        cursor_y: 0,
        document: piecetable,
    };
    enable_raw_mode();
    execute!(stdout(), EnterAlternateScreen)?;
    loop {
        editor.render()?;
    }
    disable_raw_mode();
}
