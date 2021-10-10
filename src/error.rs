use std::fmt;
use std::any;
use std::marker;


pub use Error::*;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
  Custom(String),
  Plain(Box<dyn std::error::Error>),
  SendAny(Box<dyn any::Any + marker::Send>)
}
impl Error 
{
  // build from a string
  pub fn new(string: &str) -> Self {
    Custom(String::from(string))
  }
}
impl fmt::Display for Error
{
  // display implementation
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Custom(message) => fmt::Display::fmt(message, f),
      Plain(e) => fmt::Display::fmt(&**e, f),
      SendAny(e) => fmt::Debug::fmt(&**e, f)
    }
  }
}
impl fmt::Debug for Error
{
  // debug implementation
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Custom(message) => fmt::Display::fmt(message, f),
      Plain(e) => fmt::Debug::fmt(&**e, f),
      SendAny(e) => fmt::Debug::fmt(&**e, f)    
    }
  }
}
// impl std::error::Error for Error {}
impl From<Box<dyn any::Any + marker::Send>> for Error
{
  // convert from any type
  fn from(error: Box<dyn any::Any + marker::Send>) -> Self {
    SendAny(error)
  }
}
impl From<std::io::Error> for Error
{
  // convert from io error
  fn from(error: std::io::Error) -> Self {
    Plain(Box::new(error))
  }
}
impl From<std::num::ParseIntError> for Error
{
  // conver from parse int error
  fn from(error: std::num::ParseIntError) -> Self {
    Plain(Box::new(error))
  }
}