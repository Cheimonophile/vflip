

use crate::{vflip::*, cmdui::*};


pub struct BoardComponent
{
  board: Board,
  right: Header,
  bottom: Header,
  display: Option<UIHandle>,
  loc: Option<(usize,usize)>
}


impl BoardComponent
{
  // make a new board component
  pub fn new(right: Header, bottom: Header, board: Board) -> Self  {
    BoardComponent {
      board,
      right,
      bottom,
      display: None,
      loc: None
    }
  }
}