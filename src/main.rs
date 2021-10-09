mod vflip;
mod vfio;
mod cmdui;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

const WIDTH: usize = 80;
const HEIGHT: usize = 20;

use std::io::{Read};

fn main()
{
    // create constants
    let mut right = [(0,0);vflip::SIZE];
    let mut bottom = [(0,0);vflip::SIZE];

    // set the base board
    let mut board = vflip::init();

    // give setup info
    println!("\nYour board is set up. It looks like this:\n");
    vflip::print(&board);

    // start the repl
    /* No Reps
    loop {
        match repl(&mut right, &mut bottom, &mut board)
        {
            Err(e) =>
            {
                println!("Error: {}\n", e);
            }
            Ok(true) =>
            {
                println!("Done!\n");
                break;
            }
            Ok(false) => {}
        }
    }
    */

    // get the display and the thread
    let (display, thread) = match cmdui::init(WIDTH, HEIGHT) {
        Ok(values) => values,
        Err(e) => {
            println!("{}",e);
            return;
        }
    };

    // the board
    display.render((2,3), vflip::print(&board));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // print the voltorb status
    let mut solutions = Vec::new();
    vflip::solve(&right, &bottom, board, 0, 0, &mut solutions);
    display.render((2,33), vflip::aggregate(&solutions, &board));

    // iterate over every char in stdin
    let mut buf: [u8;1] = [0];
    loop
    {
        std::io::stdin().read_exact(&mut buf);
        display.render((0,0), String::from(buf[0] as char));
    }
        


    // join with the thread
    thread.join();
}





fn repl(right: &mut vflip::Header, bottom: &mut vflip::Header, board: &mut vflip::Board) -> Res<bool>
{
    // get command from the user
    vfio::prompt("vflip> ")?;
    let command = vfio::get_command()?;

    // take an action depending on the response
    match command.as_str()
    {
        // if the command is quit
        "quit" =>
        {
            return Ok(true);
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

        // if the command is solve
        "solve" =>
        {
            println!();

            // init board storage
            let mut solutions: Vec<vflip::Board> = Vec::new();

            // solve the board
            vflip::solve(right, bottom, *board, 0, 0, &mut solutions);

            // print the number of solutions found
            println!("Found {} solutions.\n", solutions.len());
            
            // print aggregate results
            vflip::aggregate(&solutions, &board);
        }

        // if the command is board
        "board" =>
        {
            println!();

            // print the current board
            println!("Your current board:\n");
            vflip::print(board);
        }

        // if the command is set
        "set" =>
        {
            // get column
            vfio::prompt("column = ")?;
            let column = vfio::get_command()?;

            // get row
            vfio::prompt("row = ")?;
            let row = vfio::get_command()?;

            // get value
            vfio::prompt("value = ")?;
            let value = vfio::get_command()?.parse::<u8>()?;

            println!();

            // check the value
            if !vflip::VALS.contains(&value) {
                println!("Invalid Value; must be in {:?}\n", vflip::VALS);
                return Ok(false);
            }

            // create a new board with the value
            let mut new_board = *board;
            let rows = match row.as_str() {
                "all" => { 0..vflip::SIZE }
                string =>
                {
                    let index = string.parse::<usize>()?;
                    index-1..index
                }
            };
            let columns = match column.as_str() {
                "all" => { 0..vflip::SIZE }
                string =>
                {
                    let index = string.parse::<usize>()?;
                    index-1..index
                }
            };
            for row in rows.clone() {
                for column in columns.clone() {
                    new_board[row][column] = Some(value);
                }
            }
            

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
            vfio::prompt("Are you sure you want to clear the board? [y|n] ")?;
            let response = vfio::get_command()?;

            // act based on response
            match response.as_str() {
                "y" =>
                {
                    // set the board
                    *board = vflip::init();
                    println!("Board cleared.\n");
                }
                _ =>
                {
                    println!("Board not cleared.\n");
                }
            }
        }

        "set-headers" =>
        {
            // loop over the right header
            for index in 0..vflip::SIZE
            {
                // ask the user
                vfio::prompt(format!("Right {} = ", index+1).as_str())?;
                let num_string = vfio::get_command()?;
                let mut nums = num_string
                    .split_whitespace()
                    .map(|string| string.parse::<u8>());
                right[index].0 = nums.next().ok_or("")??;
                right[index].1 = nums.next().ok_or("")??;
            }

            // loop over the bottom header
            for index in 0..vflip::SIZE
            {
                // ask the user
                vfio::prompt(format!("Bottom {} = ", index+1).as_str())?;
                let num_string = vfio::get_command()?;
                let mut nums = num_string
                    .split_whitespace()
                    .map(|string| string.parse::<u8>());
                bottom[index].0 = nums.next().ok_or("")??;
                bottom[index].1 = nums.next().ok_or("")??;
            }

            // print newline
            println!();
        }


        // do nothing
        _=>{}
    }

    Ok(false)
}