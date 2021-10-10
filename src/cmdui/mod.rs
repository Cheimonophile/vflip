mod io;

use std::sync::{
  Arc,
  Mutex,
  atomic::{AtomicBool, Ordering::Relaxed}
};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use termios::*;
use termion::event::Key;
use termion::input::TermRead;
use crate::error::*;
use crate::util::*;

// constants
const STDIN_FD: std::os::unix::io::RawFd = 0;
const SLEEP_TIME: Duration  = Duration::from_millis(100);
const EMPTY: char = ' ';

// static variables
static STARTED: AtomicBool = AtomicBool::new(false);

// Starts the thread and returns its UIHandle
pub fn init(width: usize, height: usize) -> Result<UIHandle>
{
  // if the thread is started, return an error
  if STARTED.load(Relaxed) == true {
    return Err(Error::new("Display already started."));
  }

  // disable input with termios
  let mut termios = Termios::from_fd(STDIN_FD)?;
  termios.c_lflag &= !ECHO;
  termios.c_lflag &= !ICANON;
  tcsetattr(STDIN_FD, TCSANOW, &termios)?;

  // create the display
  let model = Arc::new(Mutex::new(UIModel::new(width, height)));

  // create graphical loop thread
  let thread_model = model.clone();
  let thread = thread::spawn(|| {
    // clear the screen initially
    io::clear_screen().unwrap();
    STARTED.store(true, Relaxed);
    mainloop(thread_model).unwrap();
    STARTED.store(false, Relaxed);
  });

  // return the devices
  return Ok(UIHandle::new(model, thread));
}

// renders the bits to the page
fn mainloop(model: Arc<Mutex<UIModel>>) -> Result<()>
{
  // clear the screen initially
  io::clear_screen()?;

  // start loop
  loop
  {
    // display
    io::render(&lock(&model).buffer)?;

    // set the location of the cursor, or remove the cursor
    match lock(&model).get_cursor() {
      None => { io::hide_cursor()?; }
      Some((row,column)) => {
        io::set_cursor(row, column)?;
        io::show_cursor()?;
      }
    };
    
    // sleep the thread
    thread::sleep(SLEEP_TIME);
  }
}


pub struct UIHandle {
  model: Arc<Mutex<UIModel>>,
  thread: JoinHandle<()>
}
impl UIHandle
{
  // make a new uihandle
  fn new(model: Arc<Mutex<UIModel>>, thread: JoinHandle<()>) -> Self {
    UIHandle {
      model,
      thread
    }
  }

  // get the cursor
  pub fn get_cursor(&self) -> Option<(usize, usize)> {
    lock(&self.model).get_cursor()
  }

  // set the cursor position
  pub fn set_cursor(&self, height: usize, width: usize) -> Result<()> {
    lock(&self.model).set_cursor(height, width)?;
    Ok(())
  }

  // render function
  pub fn render(&self, loc: (usize, usize), string: String) -> Result<()> {
    lock(&self.model).render(loc, string)?;
    Ok(())
  }

  // clear the display
  pub fn clear(&self) {
    lock(&self.model).clear();
  }

  // join
  pub fn join(self) -> Result<()> {
    self.thread.join()?;
    Ok(())
  }

  // get a text command and show output
  pub fn text_command(&self, loc: (usize, usize)) -> Result<String>
  {
    // init command
    let mut command = String::new();
    
    // move the cursor to loc
    self.set_cursor(loc.0, loc.1)?;
    let mut cursor = self.get_cursor().ok_or(Error::new("Cursor not set."))?;

    // iterate over chars
    for key in std::io::stdin().keys() {
      match key? {
        Key::Char('\n') =>
        {
          self.set_cursor(loc.0, loc.1)?;
          break;
        }
        Key::Char(c) => {
          cursor.1 += 1;
          self.set_cursor(cursor.0, cursor.1)?;
          command.push(c);
          self.render(loc, command.clone())?;
        }
        Key::Backspace => {
          if cursor.1 > loc.1 {
            cursor = (cursor.0, cursor.1-1);
          }
          self.render(cursor,' '.to_string())?;
          self.set_cursor(cursor.0, cursor.1)?;
          command.pop();
        }
        _ => {}
      }
    }

    // finish up
    self.set_cursor(loc.0, loc.1)?;
    self.render(cursor,' '.to_string())?;
    while cursor.1 > loc.1 {
      cursor.1 -= 1;
      self.render(cursor,' '.to_string())?;
    }


    Ok(command)
  }
}



struct UIModel {
  width: usize,
  height: usize,
  buffer: Vec<Vec<char>>,
  cursor: Option<(usize, usize)>
}
impl UIModel
{
  // Creates a new ui model with the given width and height
  fn new(width: usize, height: usize) -> Self {
    UIModel {
      width,
      height,
      buffer: vec![vec![EMPTY;width];height],
      cursor: None
    }
  }

  // returns the position of the cursor
  fn get_cursor(&self) -> Option<(usize, usize)> {
    self.cursor
  }

  // sets the cursor position
  fn set_cursor(&mut self, height: usize, width: usize) -> Result<()> {
    self.cursor = Some((height, width));
    Ok(())
  }

  // render a component
  fn render(&mut self, loc: (usize, usize), string: String) -> Result<()> {

    // init the local cursor
    let mut cursor: (usize,usize) = loc;

    // iterate over the chars
    for c in string.chars()
    {
      // if c is a whitespace char and we're in bounds, render
      if io::is_writable(c) && cursor.0 < self.height && cursor.1 < self.width {
        self.buffer[cursor.0][cursor.1] = c;
        
        // increment the cursor
        cursor = (cursor.0, cursor.1+1);
      }

      // if c is a newline character
      if c == '\n' {
        cursor = (cursor.0+1, loc.1);
      }
    }

    Ok(())
  }

  // clear the display
  fn clear(&mut self) {
    self.buffer = vec![vec![EMPTY;self.width];self.height];
  }
}