use std::io;
use std::io::Write;
use std::io::stdout;

pub fn characters(buffer: &mut String, c: char) -> io::Result<()> {
    buffer.push(c);
    write!(io::stdout(), "{}", c)?;
    stdout().flush()?;
    Ok(())
}
