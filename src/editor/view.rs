use super::terminal::{Size, Terminal};
use std::io::Error;

mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
   buffer: Buffer,
}

impl View {
   pub fn render(&self) -> Result<(), Error> {
      let Size { height, .. } = Terminal::size()?;
      for current in 0..height {
         Terminal::clear_line()?;
         if let Some(line) = self.buffer.lines.get(current) {
            Terminal::print(line)?;
            Terminal::print("\r\n")?;
            continue;
         }

         // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
         // it's allowed to be a bit up or down
         #[allow(clippy::integer_division)]
         if current == height / 2 {
            Self::draw_welcoome_message()?;
         } else {
            Self::draw_empty_row()?;
         }
         if current.saturating_add(1) < height {
            Terminal::print("\r\n")?;
         }
      }

      Ok(())
   }

   fn draw_empty_row() -> Result<(), Error> {
      Terminal::print("~")?;
      Ok(())
   }

   fn draw_welcoome_message() -> Result<(), Error> {
      let mut welcome_message = format!("{NAME} editor -- veresion {VERSION}");
      let width = Terminal::size()?.width;
      let len = welcome_message.len();
      // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
      // it's allowed to be a bit to the left or right.
      #[allow(clippy::integer_division)]
      let padding = (width.saturating_sub(len)) / 2;
      let spaces = " ".repeat(padding.saturating_sub(1));

      welcome_message = format!("~{spaces}{welcome_message}");
      welcome_message.truncate(width);
      Terminal::print(&welcome_message)?;

      Ok(())
   }
}
