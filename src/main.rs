use std::io::{self, Write};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal
};

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();

  execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

  for y in 0..40 {
    for x in 0..150 {
      if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        // in this loop we are more efficient by not flushing the buffer.
        queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( "█".magenta()))?;
      }
    }
  }
  stdout.flush()?;
  Ok(())
}