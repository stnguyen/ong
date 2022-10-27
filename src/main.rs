use crossterm::execute;
use crossterm::terminal::{size, EnterAlternateScreen, LeaveAlternateScreen};
use std::error::Error;
use std::io;

type MyError = Box<dyn Error>;

fn main() -> Result<(), MyError> {
    let (cols, rows) = size()?;
    println!("Term size {} cols, {} rows", cols, rows);

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    std::thread::sleep_ms(5000);

    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
