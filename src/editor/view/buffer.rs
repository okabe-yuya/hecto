pub struct Buffer {
   pub lines: Vec<String>
}

impl Default for Buffer {
   fn default() -> Self {
      Self {
         lines: vec![
            "Hello, World!".to_string(),
            "hecto is text editor made by Rust(full scratch)".to_string(),
         ]
      }
   }
}

