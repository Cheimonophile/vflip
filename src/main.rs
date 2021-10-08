mod vflip;

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
    let board = vflip::init();

    // solve the board
    let solved = vflip::solve(&right, &bottom, board, 0, 0);

    // determine if the board could be solved
    match solved
    {
        None => { println!("The board couldn't be solved.\n"); }
        Some(solved) => { vflip::print(&solved); }
    }

    println!("Hello, world!");
}
