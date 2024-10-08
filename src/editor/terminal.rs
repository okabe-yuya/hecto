use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Write, Error};

#[derive(Copy, Clone)]
pub struct Position {
   pub x: u16,
   pub y: u16,
}

#[derive(Copy, Clone)]
pub struct Size {
   #[allow(dead_code)]
   width: u16,
   pub height: u16,
}

pub struct Terminal {}

impl Terminal {
   pub fn terminate() -> Result<(), Error> {
      Self::execute()?;
      disable_raw_mode()?;

      Ok(())
   }

   pub fn initialize() -> Result<(), Error> {
      enable_raw_mode()?;
      Self::clear_screen()?;
      Self::move_cursor_to(Position {x:0, y:0})?;
      Self::execute()?;

      Ok(())
   }

   pub fn move_cursor_to(p: Position) -> Result<(), Error> {
      queue!(stdout(), MoveTo(p.x, p.y))?;
      Ok(())
   }

   pub fn clear_screen() -> Result<(), Error> {
      queue!(stdout(), Clear(ClearType::All))?;
      Ok(())
   }

   pub fn clear_line() -> Result<(), Error> {
      queue!(stdout(), Clear(ClearType::CurrentLine))?;
      Ok(())
   }

   pub fn hide_cursor() -> Result<(), Error> {
      queue!(stdout(), Hide)?;
      Ok(())
   }

   pub fn show_cursor() -> Result<(), Error> {
      queue!(stdout(), Show)?;
      Ok(())
   }

   pub fn print(string: &str) -> Result<(), Error> {
      queue!(stdout(), Print(string))?;
      Ok(())
   }

   pub fn size() -> Result<Size, Error> {
      let (width, height) = size()?;
      Ok(Size { width, height })
   }

   pub fn execute() -> Result<(), Error> {
      stdout().flush()?;
      Ok(())
   }
}

