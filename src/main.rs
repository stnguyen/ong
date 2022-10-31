mod document;
mod editor;
use editor::Editor;
use std::env;
use std::io;

fn main() -> Result<(), io::Error> {
    let args = env::args().collect::<Vec<String>>();
    Editor::default().run(args.get(1))?;
    Ok(())
}
