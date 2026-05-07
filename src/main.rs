use std::io::{self};
mod mode;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
mod commands;
mod input;
mod keys;
fn main() -> io::Result<()> {
    mode::state()?;
    Ok(())
}
