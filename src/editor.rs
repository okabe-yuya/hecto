use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::queue;
use std::io::{stdout, Write};

mod terminal;
use terminal::Terminal;
use terminal::Position;

pub struct Editor {
   should_quit: bool,
}

impl Editor {
   pub const fn default() -> Self {
      Self { should_quit: false }
   }

   pub fn run(&mut self) {
      Terminal::initialize().unwrap();
      let result = self.repl();
      Terminal::terminate().unwrap();
      result.unwrap();
   }

   fn draw_rows() -> Result<(), std::io::Error> {
      let height = Terminal::size()?.height;
      for current in 0..height {
         queue!(stdout(), Print("~"))?;
         if current < height - 1 {
            queue!(stdout(), Print("\r\n"))?;
         }
      }

      stdout().flush()?;
      Ok(())
   }

   fn repl(&mut self) -> Result<(), std::io::Error> {
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

   fn evaluate_event(&mut self, event: &Event) {
      if let Key(KeyEvent {
         code, modifiers, ..
      }) = event
      {
         match code {
            Char('c') if *modifiers == KeyModifiers::CONTROL => {
               self.should_quit = true;
            }
            _ => (),
         }
      }
   }

   fn refresh_screen(&self) -> Result<(), std::io::Error> {
      if self.should_quit {
         Terminal::clear_screen()?;
         print!("Goodbye.\r\n");
      } else {
         Self::draw_rows()?;
         Terminal::move_cursor_to(&Position { x: 0, y: 0 })?;
      }

      Ok(())
   }
}

