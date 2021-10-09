
// uses
use std::collections::BTreeSet;

// constants
pub const SIZE: usize = 5;
pub const VALS: [u8; 4] = [0,1,2,3];

// custom types for application
pub type Board = [[Option<u8>; SIZE]; SIZE];
pub type Label = (u8,u8);
pub type Header = [Label; SIZE];

// init the game board
pub fn init() -> Board
{
  [[None;SIZE];SIZE]
}

// validate the board
pub fn validate(right: &Header, bottom: &Header, board: &Board) -> bool
{

  // make sure each row is valid
  for row in 0..SIZE
  {
    // init the sum of the voltorbs 
    let mut num_points = 0;
    let mut num_voltorbs = 0;
    let mut num_none = 0;

    // iterate over the columns
    for column in 0..SIZE
    {
      // take action depending on the value in the cell
      match board[row][column] {
        Some(0) => { num_voltorbs += 1; }
        Some(points) => { num_points += points; }
        None => { num_none += 1; }
      }
    }

    // check if the row is valid
    if num_none > 0 {
      if num_points > right[row].0 || num_voltorbs > right[row].1 {
        return false;
      }
    }
    else {
      if num_points != right[row].0 || num_voltorbs != right[row].1 {
        return false;
      }
    }
    
  }

  // make sure each column is valid
  for column in 0..SIZE
  {
    // init the sum of the voltorbs 
    let mut num_points = 0;
    let mut num_voltorbs = 0;
    let mut num_none = 0;

    // iterate over the columns
    for row in 0..SIZE
    {
      // take action depending on the value in the cell
      match board[row][column]{
        Some(0) => { num_voltorbs += 1; }
        Some(points) => { num_points += points; }
        None => { num_none += 1; }
      }
    }

    // check if the row is valid
    if num_none > 0 {
      if num_points > bottom[column].0 || num_voltorbs > bottom[column].1 {
        return false;
      }
    }
    else {
      if num_points != bottom[column].0 || num_voltorbs != bottom[column].1 {
        return false;
      }
    }
    
  }

  // if no issues were found, return true
  return true;
}

// recursively solves the puzzle
pub fn solve(right: &Header, bottom: &Header, mut board: Board, row: usize, column: usize, solutions: &mut Vec<Board>)
{
  // base case
  if row >= SIZE || column >= SIZE
  {
    // add the board to the solutions
    solutions.push(board);

    // print the solution
    print(&board);

    // break out of the function
    return;
  }

  // check of the cell already has a value
  match board[row][column] {
    None =>
    {
      // try all values of board
      for val in VALS
      {
        // set the cell in the board
        board[row][column] = Some(val);

        // validate the board
        if validate(right, bottom, &board)
        {
          // print the intermediate board
          print(&board);

          // recursive call
          let next_column = (column + 1) % SIZE;
          let next_row = if next_column < column { row + 1 } else { row };
          solve(right, bottom, board, next_row, next_column, solutions);
        }
      }
    },
    Some(_) =>
    {
      // recursive call
      let next_column = (column + 1) % SIZE;
      let next_row = if next_column < column { row + 1 } else { row };
      return solve(right, bottom, board, next_row, next_column, solutions);
    }
  }
}


// prints the board to the screen
pub fn print(board: &Board) -> String
{

  // create a print string
  let mut print_string = String::new();
  for row in board {

    // add a space at the beginning
    print_string.push(' ');

    for cell in row {

      // push the char to the string
      print_string.push(match cell {
        Some(0) => '0',
        Some(1) => '1',
        Some(2) => '2',
        Some(3) => '3',
        Some(_) => '?',
        None => '-'
      });

      // add a space at the end
      print_string.push(' ');
    }

    // add a return in between
    print_string.push('\n');
  }

  // print the string
  //println!("{}", print_string);
  return print_string;
}



// do useful aggregation on the boards
pub fn aggregate(boards: &Vec<Board>, game_board: &Board) -> String {

  // get the number of voltorbs
  let mut num_voltorbs = [[0;SIZE];SIZE];
  let mut num_multipliers = [[0;SIZE];SIZE];

  // iterate over every board in the vector
  for board in boards {
    for row in 0..SIZE {
      for column in 0..SIZE
      {
        // get the value of the board
        match board[row][column] {
          None => {}
          Some(val) =>
          {
            // if the value is zero
            if val == 0 {
              num_voltorbs[row][column] += 1;
            }

            // if the value is greater than one
            if val > 1 {
              num_multipliers[row][column] += 1;
            }
          }
        }
      }
    }
  }

  // print the number of voltorbs
  //println!("The number of possible tables that have a voltorb in each cell:\n");
  let mut num_voltorbs_string = String::new();
  for row in 0..SIZE {
    for column in 0..SIZE
    {
      // if the game board spot is filled, don't add anything
      if game_board[row][column] != None || num_multipliers[row][column] < 1
      {
        num_voltorbs_string.push_str(format!("{: >3}", '-').as_str());
      }
      else
      {
        num_voltorbs_string.push_str(format!("{: >3}", num_voltorbs[row][column]).as_str());
      }
      num_voltorbs_string.push(' ');
    }
    num_voltorbs_string.push('\n');
  }
  //println!("{}",num_voltorbs_string);

  // print the number of multipliers
  println!("The number of possible tables that have a multiplier in each cell:\n");
  let mut num_multipliers_string = String::new();
  for row in 0..SIZE {
    for column in 0..SIZE
    {
      // if the game board spot is filled, don't add anything
      match game_board[row][column] {
        None =>
        {
          num_multipliers_string.push_str(format!("{: >3}", num_multipliers[row][column]).as_str());
        }
        Some(_) =>
        {
          num_multipliers_string.push_str(format!("{: >3}", '-').as_str());
        }
      }
      num_multipliers_string.push(' ');
    }
    num_multipliers_string.push('\n');
  }
  //println!("{}",num_multipliers_string);


  // for every cell, get its possible values
  let mut possible_values: [[BTreeSet<u8>;SIZE];SIZE] = Default::default();
  for row in 0..SIZE {
    for column in 0..SIZE {
      for board in boards
      {
        // add the cell value to the set
        match board[row][column] {
          None => {}
          Some(value) =>
          {
            possible_values[row][column].insert(value);
          }
        }
      }
    }
  }

  // print the possible values of every cell
  //println!("The possible values of every cell:\n");
  let mut possible_values_string = String::new();
  for row in possible_values {
    for cell in row
    {
      // create the string of the possible values
      let mut values_string = String::new();
      for value in cell {
        values_string.push_str(value.to_string().as_str());
      }
      possible_values_string.push_str(format!("{: >4}", values_string).as_str());
      possible_values_string.push(' ');
    }
    possible_values_string.push('\n');
  }
  //println!("{}",possible_values_string);

  // return the num_voltorbs_string
  num_voltorbs_string
}