use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    SOUTH, NORTH, WEST, EAST
}

#[derive(Clone)]
struct Node {
    access: HashSet<Direction>,
    visited: bool,
    start: bool,
    end: bool,
}

impl Node {
    pub fn new() -> Self {
        Node { access: HashSet::new(), visited: false, start: false, end: false }
    }
}

struct Maze {
    cell: Vec<Vec<Node>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Maze { cell: vec![vec![Node::new(); width]; height] }
    }

    pub fn print(&self) {
        for row in self.cell.iter() {
            for column in row.iter() {
                print!("+{}", if column.access.contains(&Direction::NORTH) {"   "} else {"---"});
            }
            print!("+");
            println!();
            for column in row.iter() {
                print!("{}   ", if column.access.contains(&Direction::WEST) {" "} else {"|"});
            }
            print!("|");
            println!();
            for column in row.iter() {
                print!("{0}{1}", if column.access.contains(&Direction::WEST) {" "} else {"|"},
                                 if column.visited {" . "} else {"   "});
            }
            print!("|");
            println!();
            for column in row.iter() {
                print!("{}   ", if column.access.contains(&Direction::WEST) {" "} else {"|"});
            }
            print!("|");
            println!();
        }

        for _ in self.cell[0].iter() {
            print!("+---");
        }
        print!("+");
        println!();
    }
}

fn main() {
    let maze: Maze = Maze::new(5, 5);
    maze.print();
}
