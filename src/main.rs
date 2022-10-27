use crossterm::event::{read, Event, KeyCode, KeyModifiers};
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
        match read()? {
            Event::Key(event) => {
                if event.modifiers.contains(KeyModifiers::CONTROL)
                    && event.code == KeyCode::Char('q')
                {
                    break;
                }
                println!("Key event {:?}", event);
            }
            _ => (),
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
