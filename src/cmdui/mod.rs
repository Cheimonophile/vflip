mod io;

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::io::{Error, ErrorKind};

extern crate termios;

use termios::*;

const STDIN_FD: std::os::unix::io::RawFd = 0;
const SLEEP_TIME: Duration  = Duration::from_millis(10);
const EMPTY: char = ' ';

type Res<T> = Result<T, Box<dyn std::error::Error>>;

// Starts the thread and returns its UIHandle
pub fn init(width: usize, height: usize) -> Res<(UIHandle, JoinHandle<()>)>
{
  // disable input with termios
  let mut termios = Termios::from_fd(STDIN_FD)?;
  termios.c_lflag &= !ECHO;
  termios.c_lflag &= !ICANON;
  tcsetattr(STDIN_FD, TCSANOW, &termios)?;

  // create the handle
  let display = UIHandle::new(width, height);

  // create graphical loop thread
  let handle = display.clone();
  let thread = thread::spawn(move || {
    mainloop(handle).unwrap();
  });

  // return the devices
  return Ok((display, thread));
}

// renders the bits to the page
fn mainloop(display: UIHandle) -> Res<()>
{
  // clear the screen initially
  io::clear_screen()?;

  // start loop
  loop
  {
    // display
    io::render(&match display.model.lock() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner()
    }.buffer)?;

    // set the location of the cursor, or remove the cursor
    match display.get_cursor()? {
      None => { io::hide_cursor()?; }
      Some((row,column)) => { io::set_cursor(row, column)?; }
    };
    thread::sleep(SLEEP_TIME);
  }

  Ok(())
}












pub struct UIHandle {
  model: Arc<Mutex<UIModel>>,
}

impl UIHandle
{
  // make a new uihandle
  fn new(width: usize, height: usize) -> Self {
    UIHandle {
      model: Arc::new(Mutex::new(UIModel::new(width, height)))
    }
  }

  // get the position of the cursor
  fn get_cursor(&self) -> Res<Option<(usize, usize)>> {
    Ok(match self.model.lock() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner()
    }.get_cursor())
  }

  // render function
  pub fn render(&self, loc: (usize, usize), string: String) {
    match self.model.lock() {
      Ok(g) => g,
      Err(p) => p.into_inner()
    }.render(loc, string);
  }

  // clear the display
  pub fn clear(&self) {
    match self.model.lock() {
      Ok(g) => g,
      Err(p) => p.into_inner()
    }.clear();
  }
}

// make the UIHandle clonable
impl Clone for UIHandle
{
  fn clone(&self) -> Self {
    UIHandle {
      model: self.model.clone()
    }
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

  // render a component
  fn render(&mut self, loc: (usize, usize), string: String) {

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
  }

  // clear the display
  fn clear(&mut self) {
    self.buffer = vec![vec![EMPTY;self.width];self.height];
  }
}