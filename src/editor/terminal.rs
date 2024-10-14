use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::{queue, Command};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Write, Error};


#[derive(Copy, Clone, Default)]
pub struct Position {
   pub col: usize,
   pub row: usize,
}

#[derive(Copy, Clone)]
pub struct Size {
   #[allow(dead_code)]
   pub width: usize,
   pub height: usize,
}

/// Represents the Terminal.
/// Edge Case for platforms where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller.
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`)
/// And should you attempt to set the caret out of these bounds, it will also be truncated.
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
      Self::execute()?;

      Ok(())
   }

   /// Moves the caret to the given Position.
   /// # Arguments
   /// * `Position` - the  `Position`to move the caret to. Will be truncated to `u16::MAX` if bigger.
   pub fn move_caret_to(p: Position) -> Result<(), Error> {
      // clippy::as_conversions: See doc adove
      #[allow(clippy::as_conversions, clippy::cast_possible_truncation)] 
      Self::queue_command(MoveTo(p.col as u16, p.row as u16))?;
      Ok(())
   }

   pub fn clear_screen() -> Result<(), Error> {
      Self::queue_command(Clear(ClearType::All))?;
      Ok(())
   }

   pub fn clear_line() -> Result<(), Error> {
      Self::queue_command(Clear(ClearType::CurrentLine))?;
      Ok(())
   }

   pub fn hide_caret() -> Result<(), Error> {
      Self::queue_command(Hide)?;
      Ok(())
   }

   pub fn show_caret() -> Result<(), Error> {
      Self::queue_command(Show)?;
      Ok(())
   }

   pub fn print(string: &str) -> Result<(), Error> {
      Self::queue_command(Print(string))?;
      Ok(())
   }

   /// Returns the current size of this Terminal.
   /// Edge Case for systems with `usize` < `u16`:
   /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
   pub fn size() -> Result<Size, Error> {
      let (width_u16, height_u16) = size()?;
      // clippy::as_conversions: See doc adove
      #[allow(clippy::as_conversions)]
      let height = height_u16 as usize;

      // clippy::as_conversions: See doc adove
      #[allow(clippy::as_conversions)]
      let width = width_u16 as usize;

      Ok(Size { width, height })
   }

   pub fn execute() -> Result<(), Error> {
      stdout().flush()?;
      Ok(())
   }

   fn queue_command<T: Command>(command: T) -> Result<(), Error> {
      queue!(stdout(), command)?;
      Ok(())
   }
}

