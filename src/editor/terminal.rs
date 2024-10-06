use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::queue;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Write};

pub struct Position {
   pub x: u16,
   pub y: u16,
}

pub struct Tsize {
   #[allow(dead_code)]
   width: u16,
   pub height: u16,
}

pub struct Terminal {}

impl Terminal {
   pub fn terminate() -> Result<(), std::io::Error> {
      disable_raw_mode()?;
      Ok(())
   }

   pub fn initialize() -> Result<(), std::io::Error> {
      enable_raw_mode()?;

      Self::hide_cursor()?;
      Self::clear_screen()?;
      stdout().flush()?;

      Self::show_cursor()?;
      Self::move_cursor_to(&Position { x: 0, y: 0 })?;
      stdout().flush()?;

      Ok(())
   }

   pub fn hide_cursor() -> Result<(), std::io::Error> {
      queue!(stdout(), Hide)
   }

   pub fn show_cursor() -> Result<(), std::io::Error> {
      queue!(stdout(), Show)
   }

   pub fn clear_screen() -> Result<(), std::io::Error> {
      queue!(stdout(), Clear(ClearType::FromCursorUp))
   }

   pub fn move_cursor_to(p: &Position) -> Result<(), std::io::Error> {
      execute!(stdout(), MoveTo(p.x, p.y))?;
      Ok(())
   }

   pub fn size() -> Result<Tsize, std::io::Error> {
      let (columns, rows) = size()?;
      Ok(Tsize {
         width: columns,
         height: rows,
      })
   }
}

