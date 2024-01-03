mod tree;
use crate::tree::*;

use rand::{seq::SliceRandom, Rng};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Direction {
    SOUTH,
    NORTH,
    WEST,
    EAST,
}

#[derive(Clone)]
pub struct Node {
    pub access: HashSet<Direction>,
    visited: bool,
    start: bool,
    end: bool,
}

impl Node {
    pub fn new() -> Self {
        Node {
            access: HashSet::new(),
            visited: false,
            start: false,
            end: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    // Direction of dst relative to src
    pub fn get_direction(&self, dst: &Position) -> Option<Direction> {
        if self.x < dst.x {
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

pub struct Maze {
    cell: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        if width < 1 || height < 1 {
            panic!("Invalid argument");
        }
        Maze {
            cell: vec![vec![Node::new(); width]; height],
            width,
            height,
        }
    }

    // TODO: Refactor into functionnal
    fn find(&self, predicate: fn(&Node) -> bool) -> Option<Position> {
        for row in self.cell.iter().enumerate() {
            for column in row.1.iter().enumerate() {
                if predicate(column.1) {
                    return Some(Position {
                        x: column.0,
                        y: row.0,
                    });
                }
            }
        }
        None
    }

    pub fn find_start(&self) -> Option<Position> {
        self.find(|e: &Node| e.start == true)
    }

    pub fn find_end(&self) -> Option<Position> {
        self.find(|e: &Node| e.end == true)
    }

    fn unvisited_neighbor(&self, position: Position) -> Vec<Position> {
        let mut buffer = Vec::new();

        if position.x > 0 {
            if !self.cell[position.y][position.x - 1].visited {
                buffer.push(Position {
                    x: position.x - 1,
                    y: position.y,
                });
            }
        }
        if position.x < self.width - 1 {
            if !self.cell[position.y][position.x + 1].visited {
                buffer.push(Position {
                    x: position.x + 1,
                    y: position.y,
                });
            }
        }
        if position.y > 0 {
            if !self.cell[position.y - 1][position.x].visited {
                buffer.push(Position {
                    x: position.x,
                    y: position.y - 1,
                });
            }
        }
        if position.y < self.height - 1 {
            if !self.cell[position.y + 1][position.x].visited {
                buffer.push(Position {
                    x: position.x,
                    y: position.y + 1,
                });
            }
        }

        buffer
    }

    pub fn is_dead_end(&self, position: Position) -> bool {
        todo!()
    }

    pub fn make(&mut self) {
        let mut previous_position = Position {
            x: rand::thread_rng().gen_range(0..self.width),
            y: rand::thread_rng().gen_range(0..self.height),
        };
        let mut stack: VecDeque<Position> = VecDeque::new();
        stack.push_front(previous_position);
        self.cell[previous_position.y][previous_position.x].visited = true;
        self.cell[previous_position.y][previous_position.x].start = true;

        let mut is_first_time = true;

        while !stack.is_empty() {
            previous_position = *stack.front().unwrap();
            let buffer = self.unvisited_neighbor(previous_position);

            match buffer.choose(&mut rand::thread_rng()) {
                Some(&next_position) => {
                    // Break walls
                    self.cell[previous_position.y][previous_position.x]
                        .access
                        .insert(previous_position.get_direction(&next_position).unwrap());
                    self.cell[next_position.y][next_position.x]
                        .access
                        .insert(next_position.get_direction(&previous_position).unwrap());

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

    pub fn get_distance(&self, position: Position) -> usize {
        let end = self.find_end().unwrap();
        end.x.abs_diff(position.x) + end.y.abs_diff(position.y)
    }

    pub fn get_node_from_position(&self, position: Position) -> Option<Node> {
        if position.x < self.width && position.y < self.height {
            return Some(self.cell[position.y][position.x].clone());
        }
        None
    }

    pub fn mark_position_visited(&mut self, position: Position) {
        if position.x < self.width && position.y < self.height {
            self.cell[position.y][position.x].visited = true;
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in self.cell.iter() {
            for column in row.iter() {
                write!(
                    f,
                    "+{}",
                    if column.access.contains(&Direction::NORTH) {
                        "   "
                    } else {
                        "---"
                    }
                )?;
            }
            write!(f, "+\n")?;
            for column in row.iter() {
                write!(
                    f,
                    "{0}{1}",
                    if column.access.contains(&Direction::WEST) {
                        " "
                    } else {
                        "|"
                    },
                    if column.start || column.end {
                        " o "
                    } else if column.visited {
                        " . "
                    } else {
                        "   "
                    }
                )?;
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

    let mut tree = Tree::new();

    let mut node = TreeNode::new(maze.find_start().unwrap());
    node.calculate_heuristic(&maze);
    tree.insert(&mut node);
    println!("{}", tree);
}
