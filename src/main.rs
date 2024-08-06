use crate::streamhandler::StreamHandler;

use crossterm::{
    event::{KeyCode, KeyModifiers},
    style::Print,
    *,
};

mod streamhandler;
mod ui;

fn main() -> std::io::Result<()> {
    let (width, height) = terminal::size()?;
    let mut stream = StreamHandler::new(std::io::stdout(), width, height);

    stream.start()?;

    loop {
        stream.execute(cursor::MoveTo(0, 0))?;
        stream.execute(Print(ui::ui_string()))?;
        match event::read()? {
            event::Event::Mouse(event) => {
                ui::handle_mouse_event(&mut stream, event)?;
            }
            event::Event::Key(key) => {
                if let event::KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } = key
                {
                    break;
                }

                if ui::is_in_text_field() {
                    ui::handle_text_input(&mut stream, key)?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
