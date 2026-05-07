use std::io::{self};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
mod input;
mod keys;
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    input::input();
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
