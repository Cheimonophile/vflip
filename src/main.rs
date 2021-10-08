mod vflip;
mod vfio;

type R<T> = Result<T, Box<dyn std::error::Error>>;

fn main()
{
    // create constants
    let right = [
        (4,2),
        (6,1),
        (5,1),
        (4,1),
        (5,1)
    ];
    let bottom = [
        (5,1),
        (4,3),
        (5,1),
        (5,1),
        (5,0)
    ];

    // set the base board
    let mut board = vflip::init();

    // give setup info
    println!("\nYour board is set up. It looks like this:\n");
    vflip::print(&board);

    // start the repl
    loop {
        match repl(&right, &bottom, &mut board)
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
}





fn repl(right: &vflip::Header, bottom: &vflip::Header, board: &mut vflip::Board) -> R<bool>
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
            println!("\tCommands:");
            println!("help");
            println!("quit");
            println!("solve");
            println!("board");
            println!("set");
            println!();
        }

        // if the command is solve
        "solve" =>
        {
            // init board storage
            let mut solutions: Vec<vflip::Board> = Vec::new();

            // solve the board
            vflip::solve(right, bottom, *board, 0, 0, &mut solutions);

            // print the number of solutions found
            println!("Found {} solutions.\n", solutions.len());
        }

        // if the command is board
        "board" =>
        {
            // print the current board
            println!("Your current board:\n");
            vflip::print(board);
        }

        // if the command is set
        "set" =>
        {
            // get column
            vfio::prompt("column = ")?;
            let column = vfio::get_command()?.parse::<usize>()?;

            // get row
            vfio::prompt("row = ")?;
            let row = vfio::get_command()?.parse::<usize>()?;

            // get value
            vfio::prompt("value = ")?;
            let value = vfio::get_command()?.parse::<u8>()?;

            // check the value
            if !vflip::VALS.contains(&value) {
                println!("Invalid Value\n");
                return Ok(false);
            }

            // create a new board with the value
            let mut new_board = *board;
            new_board[vflip::SIZE-row][column-1] = Some(value);

            // ask the user if they're sure they want to change it
            println!("Your new board looks like this:\n");
            vflip::print(&new_board);
            vfio::prompt("Are you sure you want to change it? [y|n] ")?;
            let response = vfio::get_command()?;

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


        // do nothing
        _=>{}
    }

    Ok(false)
}