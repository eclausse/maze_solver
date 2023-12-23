use std::{collections::{HashSet, VecDeque}, fmt::Display};
use rand::{Rng, seq::SliceRandom};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    // Direction of dst relative to src
    pub fn get_direction(&self, dst: &Position) -> Option<Direction> {
        if self.x < dst.x{
            return Some(Direction::EAST);
        }

        if self.x > dst.x {
            return Some(Direction::WEST);
        }

        if self.y < dst.y {
            return Some(Direction::SOUTH);
        }

        if self.y > dst.y {
            return Some(Direction::NORTH);
        }

        None
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

    fn unvisited_neighbor(&self, x: usize, y: usize) -> Vec<Position> {
        let mut buffer = Vec::new();

        if x > 0 {
            if !self.cell[y][x-1].visited {
                buffer.push(Position { x: x-1, y});
            }
        }
        if x < self.width - 1 {
            if !self.cell[y][x+1].visited {
                buffer.push(Position { x: x+1, y});
            }
        }
        if y > 0 {
            if !self.cell[y-1][x].visited {
                buffer.push(Position { x, y: y-1});
            }
        }
        if y < self.height - 1 {
            if !self.cell[y+1][x].visited {
                buffer.push(Position { x, y: y+1});
            }
        }

        buffer
    }

    pub fn make(&mut self) {
        let mut previous_position = Position {
            x: rand::thread_rng().gen_range(0..self.width),
            y: rand::thread_rng().gen_range(0..self.height)
        };
        let mut stack: VecDeque<Position> = VecDeque::new();
        stack.push_front(previous_position);
        self.cell[previous_position.y][previous_position.x].visited = true;
        self.cell[previous_position.y][previous_position.x].start = true;

        let mut is_first_time = true;

        while !stack.is_empty() {
            previous_position = *stack.front().unwrap();
            let buffer = self.unvisited_neighbor(previous_position.x, previous_position.y);

            match buffer.choose(&mut rand::thread_rng()) {
                Some(&next_position) => {
                    // Break walls
                    self.cell[previous_position.y][previous_position.x].access.insert(previous_position.get_direction(&next_position).unwrap());
                    self.cell[next_position.y][next_position.x].access.insert(next_position.get_direction(&previous_position).unwrap());

                    // Mark node as visited
                    self.cell[next_position.y][next_position.x].visited = true;

                    // Push into stack
                    stack.push_front(next_position.clone());
                }
                None => {
                    if is_first_time {
                        is_first_time = !is_first_time;
                        self.cell[previous_position.y][previous_position.x].end = true;
                    }
                    // Back-trace
                    stack.pop_front();
                }
            }
        }

        for ite in self.cell.iter_mut() {
            ite.iter_mut().for_each(|ele| ele.visited = false);
        }
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
    let mut maze: Maze = Maze::new(10, 10);
    maze.make();
    println!("{maze}");
    // println!("{:#?}", maze.cell[4][4].access);
}
