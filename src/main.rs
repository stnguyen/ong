use crossterm::event::{read, Event};
use crossterm::terminal::{size, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use std::error::Error;
use std::io;

type MyError = Box<dyn Error>;

fn main() -> Result<(), MyError> {
    let (cols, rows) = size()?;
    println!("Term size {} cols, {} rows", cols, rows);

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    loop {
        println!("Read: {:?}", read()?);
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
