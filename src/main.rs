mod maze;
mod tree;
use std::env;

use crate::maze::Maze;

fn main() {
    let (width, height) = parse_arguments();

    let mut maze: Maze = Maze::new(width, height);
    maze.make();
    maze.solve();
    println!("{maze}");
}

fn parse_arguments() -> (usize, usize) {
    let args: Vec<String> = env::args().collect();
    (parse(args.get(1)), parse(args.get(2))) // TO LOOK: Is there a functional combinator g.f (x, y) -> (f(x), f(y)) ?
}

fn parse(arg: Option<&String>) -> usize {
    match arg {
        None => return 10,
        Some(h) => {
            let res = h.parse::<usize>().expect(
                "Argument can only be positive number superior to 3 \n Usage: cargo run -- [width height]",
            );
            if res < 3 {
                panic!("Argument can only be positive number superior to 3 \n Usage: cargo run -- [width height]");
            }
            res
        }
    }
}
