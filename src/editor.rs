use std::io::Error;

use core::cmp::min;
use crossterm::event::{
   read,
   Event::{self, Key},
   KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
mod terminal;
mod view;

use terminal::{Terminal, Position, Size};
use view::View;

#[derive(Copy, Clone, Default)]
struct Location {
   x: usize,
   y: usize,
}

#[derive(Default)]
pub struct Editor {
   should_quit: bool,
   location: Location, 
   view: View,
}

impl Editor {
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
         self.evaluate_event(&event)?;
      }
      Ok(())
   }

   fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
      let Location { mut x, mut y } = self.location;
      let Size { height, width } = Terminal::size()?;
      match key_code {
         KeyCode::Up | KeyCode::Char('k') => {
            y = y.saturating_sub(1);
         }
         KeyCode::Down | KeyCode::Char('j') => {
            y = min(height.saturating_sub(1), y.saturating_add(1));
         }
         KeyCode::Left | KeyCode::Char('h') => {
            x = x.saturating_sub(1);
         }
         KeyCode::Right | KeyCode::Char('l') => {
            x = min(width.saturating_sub(1), x.saturating_add(1));
         }
         KeyCode::PageUp => {
            y = 0;
         }
         KeyCode::PageDown => {
            y = height.saturating_sub(1);
         }
         KeyCode::Home => {
            x = 0;
         }
         KeyCode::End => {
            x = width.saturating_sub(1);
         }
         _ => (),
      }

      self.location = Location { x, y };
      Ok(())
   }

   fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
      if let Key(KeyEvent {
         code, modifiers, kind: KeyEventKind::Press, ..
      }) = event
      {
         match code {
            KeyCode::Char('c') if *modifiers == KeyModifiers::CONTROL => {
               self.should_quit = true;
            }
            KeyCode::Up
            | KeyCode::Down
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::PageDown
            | KeyCode::PageUp
            | KeyCode::End
            | KeyCode::Home
            | KeyCode::Char('j' | 'k' | 'h' | 'l') => {
               self.move_point(*code)?;
            }
            _ => (),
         }
      }
      
      Ok(())
   }

   fn refresh_screen(&mut self) -> Result<(), Error> {
      Terminal::hide_caret()?;
      Terminal::move_caret_to(Position::default())?;
      if self.should_quit {
         Terminal::clear_screen()?;
         Terminal::print("Goodbye.\r\n")?;
      } else {
         self.view.render()?;
         Terminal::move_caret_to(Position {
            col: self.location.x,
            row: self.location.y,
         })?;
      }

      Terminal::show_caret()?;
      Terminal::execute()?;
      Ok(())
   }
}

