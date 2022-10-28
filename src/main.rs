mod editor;
use editor::Editor;
use std::io;

fn main() -> Result<(), io::Error> {
    Editor::default().run()?;
    Ok(())
}
