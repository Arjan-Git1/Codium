use crossterm::{
    cursor::{self, MoveTo},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, stdout};

use crate::{commands, input};
#[derive(Debug)]
pub enum Mode {
    Editing,
    Normal,
    Selection,
    Quit,
}
pub fn state() -> io::Result<()> {
    let mut mode = Mode::Normal;
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    loop {
        match &mode {
            Mode::Editing => {
                input::input(&mut mode)?;
            }
            Mode::Normal => {
                commands::commands(&mut mode)?;
            }
            Mode::Selection => {}

            Mode::Quit => {
                break;
            }
        }
    }
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
