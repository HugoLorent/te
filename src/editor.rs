mod terminal;
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers,
};
use std::io::Error;
use terminal::{Position, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Self::draw_welcome_message()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn draw_rows() -> Result<(), Error> {
        let terminal_height = Terminal::size()?.height;
        for current_row in 0..terminal_height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_row + 1 < terminal_height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let message_width = welcome_message.len() as u16;
        let terminal_size = Terminal::size()?;
        let x = (terminal_size.width - message_width) / 2;
        let y = terminal_size.height / 3;
        Terminal::move_cursor_to(Position { x, y })?;
        Terminal::print(&welcome_message)?;
        Ok(())
    }
}
