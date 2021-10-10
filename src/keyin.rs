use std::io::Stdin;

const ESC: char = 27 as char;

// return if a char is writable
pub fn is_writable(c: char) -> bool {
  c as u8 > 31 && (c as u8) < 127
}



pub struct Keyin {
  stdin: Stdin
}
impl Iterator for Keyin
{
  // iterator item
  type Item = Key;

  // get the next key
  fn next(&mut self) -> Option<Self::Item>
  {

  }
}


pub enum Key {
  Up,
  Down,
  Left,
  Right,
  Writable(char)
}