use crossterm::{
    
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

