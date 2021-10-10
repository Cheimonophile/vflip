mod vflip;
mod vfio;
mod cmdui;
mod vflipuic;
mod error;
mod util;
//mod keyin;


use crate::error::*;

const WIDTH: usize = 80;
const HEIGHT: usize = 20;
const BOARD_LOC: (usize, usize) = (2,3);
const STATUS_LOC: (usize, usize) = (2,33);
const NEXT_LOC: (usize, usize) = (12,3);
const CURSOR_DEFAULT: (usize, usize) = (HEIGHT-1, 1);
const QUESTION_LOC: (usize,usize) = (HEIGHT-2, 1);

fn main() -> Result<()>
{
    // create constants
    let mut right = [(0,0);5];
    let mut bottom = [(0,0);5];

    // set the base board
    let mut board = vflip::init();

    // give setup info
    println!("\nYour board is set up. It looks like this:\n");
    vflip::print_with_headers(&right, &bottom, &board);

    // get the display and the thread
    let display = cmdui::init(WIDTH, HEIGHT)?;

    // the board
    display.render((2,3), vflip::print_with_headers(&right, &bottom, &board))?;
    std::thread::sleep(std::time::Duration::from_millis(100));

    // print the voltorb status
    let mut solutions = Vec::new();
    vflip::solve(&right, &bottom, board, 0, 0, &mut solutions);
    display.render(STATUS_LOC, vflip::aggregate(&solutions, &board))?;

    // set data for loop
    display.set_cursor(CURSOR_DEFAULT.0, CURSOR_DEFAULT.1)?;
    let mut possibilities = String::new();

    // iterate over every char in stdin
    loop
    {
        // clear the board
        display.clear();

        // the board
        display.render(BOARD_LOC, vflip::print_with_headers(&right, &bottom, &board))?;

        // print the voltorb status
        let mut solutions = Vec::new();
        vflip::solve(&right, &bottom, board, 0, 0, &mut solutions);
        display.render(STATUS_LOC, vflip::aggregate(&solutions, &board))?;

        // get a command from the key
        let command = display.text_command(CURSOR_DEFAULT)?;
        if command.as_str() == "quit" { break; }
        process_command(
            &mut right,
            &mut bottom, 
            &mut board, 
            &display,
            &command,
            &mut possibilities
        )?;
    }

    Ok(())
}



fn process_command(
    right: &mut vflip::Header,
    bottom: &mut vflip::Header,
    board: &mut vflip::Board,
    display: &cmdui::UIHandle,
    command: &String,
    possibilities: &mut String
) -> Result<()>
{
    // turn the command into an iterator
    let mut command_iter = command.split_whitespace();

    // take an action depending on the response
    match command_iter.next().ok_or(Error::new("Unparsable"))?
    {
        // if the command is quit
        "quit" =>
        {
            return Ok(());
        }

        // if the command is help
        "help" =>
        {
            println!();
            println!("\tCommands:");
            println!("help");
            println!("quit");
            println!("solve");
            println!("board");
            println!("set");
            println!("reset");
            println!("clear");
            println!("headers");
            println!();
        }

        // if the command is set
        "set" =>
        {
            // get column
            let column = match command_iter.next() {
                Some("all") => "0",
                Some(val) => val,
                None => { return Ok(()); }
            }.parse::<u8>()?;

            // get row
            let row = match command_iter.next() {
                Some("all") => "0",
                Some(val) => val,
                None => { return Ok(()); }
            }.parse::<u8>()?;

            // get value
            let value = match command_iter.next() {
                Some(val) => val,
                None => { return Ok(()); }
            }.parse::<u8>()?;

            //println!();

            // check the value
            if !vflip::VALS.contains(&value) {
                display.render(CURSOR_DEFAULT,format!("Invalid Value; must be in {:?}\n", vflip::VALS))?;
                return Ok(());
            }

            // create a new board with the value
            let mut new_board = *board;
            let rows = match row {
                0 => 0..vflip::SIZE,
                val =>
                {
                    let index = val as usize;
                    index-1..index
                }
            };
            let columns = match column {
                0 => 0..vflip::SIZE,
                val =>
                {
                    let index = val as usize;
                    index-1..index
                }
            };
            for row in rows.clone() {
                for column in columns.clone() {
                    new_board[row][column] = Some(value);
                }
            }
            

            // ask the user if they're sure they want to change it
            display.render(NEXT_LOC, vflip::print(&new_board))?;
            display.render(QUESTION_LOC, "Are you sure you want to change it? [y|n] ".to_owned())?;
            let response = display.text_command(CURSOR_DEFAULT)?;
            //println!();

            // act based on response
            match response.as_str() {
                "y" =>
                {
                    // set the board
                    *board = new_board;
                    //println!("New board set.\n");
                }
                _ =>
                {
                    //println!("New board not set.\n");
                }
            }
        }

        // reset a single cell on the board
        "reset" =>
        {
            // get column
            vfio::prompt("column = ")?;
            let column = vfio::get_command()?.parse::<usize>()?;

            // get row
            vfio::prompt("row = ")?;
            let row = vfio::get_command()?.parse::<usize>()?;

            println!();

            // create a new board, setting the cell to none
            let mut new_board = *board;
            new_board[vflip::SIZE-row][column-1] = None;

            // ask the user if they're sure they want to change it
            println!("Your new board looks like this:\n");
            vflip::print(&new_board);
            vfio::prompt("Are you sure you want to change it? [y|n] ")?;
            let response = vfio::get_command()?;
            println!();

            // act based on response
            match response.as_str() {
                "y" =>
                {
                    // set the board
                    *board = new_board;
                    println!("New board set.\n");
                }
                _ =>
                {
                    println!("New board not set.\n");
                }
            }
        }

        // clear
        "clear" =>
        {
            // ask the user if they really want to clear the board
            display.render(QUESTION_LOC,"Are you sure you want to clear the board? [y|n] ".to_string())?;
            let response = display.text_command(CURSOR_DEFAULT)?;

            // act based on response
            match response.as_str() {
                "y" =>
                {
                    // set the board
                    *board = vflip::init();
                    //println!("Board cleared.\n");
                }
                _ =>
                {
                    //println!("Board not cleared.\n");
                }
            }
        }

        "headers" =>
        {
            // loop over the right header
            for index in 0..vflip::SIZE
            {
                // ask the user
                display.render(QUESTION_LOC, format!("Right {} =    ", index+1))?;
                let num_string = display.text_command(CURSOR_DEFAULT)?;
                let mut nums = num_string
                    .split_whitespace()
                    .map(|string| string.parse::<u8>());
                right[index].0 = nums.next().ok_or(Error::new("Bad Arguments"))??;
                right[index].1 = nums.next().ok_or(Error::new("Bad Arguments"))??;
            }

            // loop over the bottom header
            for index in 0..vflip::SIZE
            {
                // ask the user
                display.render(QUESTION_LOC, format!("Bottom {} =     ", index+1))?;
                let num_string = display.text_command(CURSOR_DEFAULT)?;
                let mut nums = num_string
                    .split_whitespace()
                    .map(|string| string.parse::<u8>());
                bottom[index].0 = nums.next().ok_or(Error::new("Bad Arguments"))??;
                bottom[index].1 = nums.next().ok_or(Error::new("Bad Arguments"))??;
            }

            // print newline
            //println!();
        }


        // do nothing
        _=>{}
    }

    Ok(())
}