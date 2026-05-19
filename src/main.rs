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

use crate::{input::input, mode::Mode};
use crossterm::{
    ExecutableCommand,
    event::KeyCode,
    execute,
    terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use editor::Editor;
use io::stdout;
use std::path::Path;
fn main() -> io::Result<()> {
    let path = Path::new("text.txt");

    let document = PieceTable::from_file(path).unwrap();
    let mut editor = Editor {
        mode: Mode::Normal,
        cursor_x: 0,
        cursor_y: 0,
        document: document,
    };
    enable_raw_mode();
    execute!(stdout(), EnterAlternateScreen)?;
    loop {
        let code = input(&mut editor.mode);
        editor.render()?;
        editor.input(&code);
        match &code {
            Ok(KeyCode::Char('I')) => {
                editor.mode = Mode::Editing;
            }
            Ok(KeyCode::Esc) => {
                editor.mode = Mode::Normal;
            }

            Ok(_) => {}
            Err(e) => {}
        }
        match editor.mode {
            Mode::Editing => editor.insert(&code),
            Mode::Normal => {
                editor.normal(&code);
            }
            Mode::Selection => {}
            Mode::Quit => {
                break;
            }
        }
    }
    disable_raw_mode()
}
