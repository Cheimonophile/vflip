use std::io::{self, Write};


pub fn get_command() -> io::Result<String>
{
  // get the command from the user
  let mut command = String::new();
  io::stdin().read_line(&mut command)?;

  // cleanup the string
  command = String::from(command.trim()).to_lowercase();

  // return ok
  Ok(command)
}

// prints the prompt
pub fn prompt(prompt: &str) -> io::Result<()> {
  print!("{}",prompt);
  io::stdout().flush()?;
  Ok(())
}