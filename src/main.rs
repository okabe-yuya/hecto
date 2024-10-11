#![warn(clippy::all, clippy::pedantic, clippy::arithmetic_side_effects, clippy::as_conversions, clippy::integer_division)]
mod editor;
use editor::Editor;

fn main() {
   Editor::default().run();
}

