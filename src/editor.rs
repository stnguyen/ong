use crossterm::{cursor, event, execute, terminal};
use event::{Event, KeyCode, KeyModifiers};
use std::io;

pub struct Editor {
    cols: u16,
    rows: u16,
}

impl Editor {
    pub fn run(&mut self) -> Result<(), io::Error> {
        self.start()?;

        // Infinite loop
        loop {
            self.draw_rows();

            // event::read() is blocking until an Event is triggered
            match event::read()? {
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

        self.stop()
    }

    fn start(&mut self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            cursor::EnableBlinking
        )?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        terminal::disable_raw_mode()?;
        execute!(stdout, terminal::LeaveAlternateScreen)?;
        println!("Bye!");
        Ok(())
    }

    fn draw_rows(&self) {
        for r in 1..self.rows {
            if r == self.rows / 3 {
                let hello_msg = "Hello world!";
                let hello_msg = format!(
                    "{}{}",
                    " ".repeat(((self.cols - 1) as usize - hello_msg.len()) / 2),
                    hello_msg
                );
                println!("~{}\r", hello_msg);
            } else {
                println!("~\r");
            }
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        let (cols, rows) = terminal::size().expect("Cannot instantiate Editor");
        Self { cols, rows }
    }
}
