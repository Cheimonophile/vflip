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

    // create vector to store the solutions
    let mut solutions: Vec<vflip::Board> = Vec::new();

    // print solving
    println!("\nSolving...\n");

    // solve the board
    vflip::solve(&right, &bottom, board, 0, 0, &mut solutions);

    // print done
    println!("Done!\n");
}
