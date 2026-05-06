use std::{
    fmt::write,
    io::{self, Write, stdout},
};

use crossterm::{
    cursor::{self, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut buffer = String::new();
    loop {
        if let Event::Key(event) = event::read()? {
            if event.kind == KeyEventKind::Press {
                match (event.modifiers, event.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                        break;
                    }

                    (_, KeyCode::Char(c)) => {
                        buffer.push(c);
                        write!(io::stdout(), "{}", buffer)?;
                        stdout().flush()?;
                        buffer.clear();
                    }
                    (_, KeyCode::Enter) => {
                        println!();
                        execute!(stdout(), MoveToNextLine(2))?;
                    }
                    //next day i have to add backspace and make code modules as the code is bhery
                    //messsi iamsocoolmaneveryonelikesmeiamtheworstcoderofalltime
                    _ => {}
                }
            }
        }
    }
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
