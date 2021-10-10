
use std::io::{Write};
use crate::error::*;

const ESC: char = 27 as char;

// return if a char is writable
pub fn is_writable(c: char) -> bool {
  c as u8 > 31 && (c as u8) < 127
}

// print something to the screen
fn print(c: char) -> Result<()> {
  print!("{}",c);
  std::io::stdout().flush()?;
  Ok(())
}

// clear the screen
pub fn clear_screen() -> Result<()> {
  for c in format!("{esc}[1;1H{esc}[3J{esc}[0J",esc=ESC).chars() {
    print(c)?;
  };
  Ok(())
}

// hide cursor
pub fn hide_cursor() -> Result<()> {
  for c in format!("{esc}[?25l",esc=ESC).chars() {
    print(c)?;
  };
  Ok(())
}

// show cursor
pub fn show_cursor() -> Result<()> {
  for c in format!("{esc}[?25h",esc=ESC).chars() {
    print(c)?;
  };
  Ok(())
}

// set the cursor location
pub fn set_cursor(row: usize, column: usize) -> Result<()> {
  for c in format!("{esc}[{row};{column}H",esc=ESC,row=row,column=column).chars() {
    print(c)?;
  };
  Ok(())
}

// render function
pub fn render(buffer: &Vec<Vec<char>>) -> Result<()> {

  // make sure cursor at zero and is hidden
  hide_cursor()?;
  
  // make sure the characters are whitespace
  if !buffer.iter().all(|row| row.iter().all(|cell| is_writable(*cell))) {
    return Err(Error::new("Invalid char in screen buffer."));
  }

  // print the chars to the screen
  for row in 0..buffer.len() {
    for column in 0..buffer[row].len() {
      set_cursor(row,column)?;
      print(buffer[row][column])?;
    }
  }

  Ok(())
}
