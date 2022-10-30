use crossterm::{cursor, event, execute, terminal};
use event::{Event, KeyCode, KeyEvent, KeyModifiers};
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
            self.refresh_screen()?;

            // event::read() is blocking until an Event is triggered
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL)
                        && key_event.code == KeyCode::Char('q')
                    {
                        break;
                    }

                    self.process_key(key_event)?;
                }
                _ => (),
            }
        }

        self.stop()
    }

    fn start(&mut self) -> Result<(), io::Error> {
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
        )?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), io::Error> {
        terminal::disable_raw_mode()?;
        execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
        println!("Bye!");
        Ok(())
    }

    fn process_key(&self, key_event: KeyEvent) -> Result<(), io::Error> {
        // print!("{:?}", key_event);
        match key_event.code {
            KeyCode::Up => execute!(io::stdout(), cursor::MoveUp(1)),
            KeyCode::Down => execute!(io::stdout(), cursor::MoveDown(1)),
            KeyCode::Left => execute!(io::stdout(), cursor::MoveLeft(1)),
            KeyCode::Right => execute!(io::stdout(), cursor::MoveRight(1)),

            // Do nothing
            _ => Ok(()),
        }
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        execute!(
            stdout,
            cursor::SavePosition,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )?;
        self.draw_rows();
        execute!(stdout, cursor::RestorePosition,)?;
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
