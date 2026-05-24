use std::{
    io::{self},
    vec,
};
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

mod commands;
mod editor;
mod input;
mod keys;
mod mode;
use crate::{input::input, mode::Mode};
use crossterm::{
    ExecutableCommand,
    event::KeyCode,
    execute,
    terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use editor::Editor;
use io::stdout;
use piecetable::PieceTable;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
};
use std::path::Path;
fn main() -> io::Result<()> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let paths: String = String::from("editor.rs");
    let path = Path::new(&paths);

    let document = PieceTable::from_file(path).unwrap();
    let mut editor = Editor {
        mode: Mode::Normal,
        cursor_x: 0,
        cursor_y: 0,
        document: document,
        scroll_x: 0,
        scroll_y: 0,
        visible_height: 0,
        syntax_set: syntax_set,
        theme_set: theme_set,
    };
    enable_raw_mode();
    execute!(stdout(), EnterAlternateScreen)?;
    loop {
        let code = input(&mut editor.mode);
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
                editor.normal(&code, &path);
            }
            Mode::Selection => {}
            Mode::Quit => {
                break;
            }
        }

        editor.render(&mut terminal, &paths)?;
    }
    disable_raw_mode()
}
