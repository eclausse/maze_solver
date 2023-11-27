use std::{collections::{HashSet, VecDeque}, fmt::Display};

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
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        if width < 1 || height < 1 {
            panic!("Invalid argument");
        }
        Maze { cell: vec![vec![Node::new(); width]; height], width, height }
    }

    
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        for row in self.cell.iter() {
            for column in row.iter() {
                write!(f, "+{}", if column.access.contains(&Direction::NORTH) {"   "} else {"---"})?;
            }
            write!(f, "+\n")?;
            for column in row.iter() {
                write!(f, "{0}{1}", if column.access.contains(&Direction::WEST) {" "} else {"|"},
                                 if column.start || column.end {" o "} else if column.visited {" . "} else {"   "})?;
            }
            write!(f, "|\n")?;
        }

        for _ in self.cell[0].iter() {
            write!(f, "+---")?;
        }
        write!(f, "+\n")
    }
}

fn main() {
    let mut maze: Maze = Maze::new(5, 5);
    maze.make();
    println!("{maze}");
}
