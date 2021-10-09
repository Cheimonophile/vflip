
use std::io::{Write, Error, ErrorKind};

const ESC: char = 27 as char;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

// return if a char is writable
pub fn is_writable(c: char) -> bool {
  c as u8 > 31 && (c as u8) < 127
}

// print something to the screen
fn print(c: char) -> Res<()> {
  print!("{}",c);
  std::io::stdout().flush()?;
  Ok(())
}

// clear the screen
pub fn clear_screen() -> Res<()> {
  for c in format!("{esc}[1;1H{esc}[3J{esc}[0J",esc=ESC).chars() {
    print(c)?;
  };
  Ok(())
}

// hide cursor
pub fn hide_cursor() -> Res<()> {
  for c in format!("{esc}[?25l",esc=ESC).chars() {
    print(c)?;
  };
  Ok(())
}

// show cursor
pub fn show_cursor() -> Res<()> {
  for c in format!("{esc}[?25h",esc=ESC).chars() {
    print(c)?;
  };
  Ok(())
}

// set the cursor location
pub fn set_cursor(row: usize, column: usize) -> Res<()> {
  for c in format!("{esc}[{row};{column}H",esc=ESC,row=row,column=column).chars() {
    print(c)?;
  };
  Ok(())
}

// render function
pub fn render(buffer: &Vec<Vec<char>>) -> Res<()> {

  // make sure cursor at zero and is hidden
  hide_cursor()?;
  
  // make sure the characters are whitespace
  if !buffer.iter().all(|row| row.iter().all(|cell| is_writable(*cell))) {
    return Err(Box::new(Error::new(ErrorKind::InvalidInput, "Invalid char in screen buffer.")));
  }

  // print the chars to the screen
  for row in 0..buffer.len() {
    for column in 0..buffer[row].len() {
      set_cursor(row+1,column+1)?;
      print(buffer[row][column])?;
    }
  }

  // reshow the cursor
  show_cursor()?;

  Ok(())
}
