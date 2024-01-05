use crate::maze::*;

use std::fmt::Display;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type TreeNodePtr = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
pub struct TreeNode {
    parent: Option<Weak<RefCell<TreeNode>>>,
    childs: Vec<TreeNodePtr>,
    position: Position,
    dead_end: bool,
    f: usize,
    g: usize,
    h: usize,
}

impl TreeNode {
    pub fn new(position: Position) -> Self {
        TreeNode {
            parent: None,
            childs: Vec::new(),
            position,
            dead_end: false,
            f: 0, // Sum g + h
            g: 0, // Depth in tree
            h: 0, // Heuristic score
        }
    }

    // Calculate h and recompute f
    pub fn calculate_heuristic(&mut self, maze: &Maze) {
        let end = maze.find_end().unwrap();
        self.h = end.x.abs_diff(self.position.x) + end.y.abs_diff(self.position.y);
        self.f = self.g + self.h;
    }

    // Insert a node and update g and f
    pub fn insert_node(node: &mut TreeNodePtr, to_insert: &mut TreeNode) {
        // Update parent node
        to_insert.parent = Some(Rc::downgrade(&node));

        // Update values
        to_insert.g = node.as_ref().borrow().g + 1;
        to_insert.f = to_insert.g + to_insert.h;

        // Insert
        node.borrow_mut()
            .childs
            .push(Rc::new(RefCell::new(to_insert.to_owned())));
    }

    fn best_valid_position_recursion(
        node: &Rc<RefCell<TreeNode>>,
    ) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
        let current = node.as_ref().borrow();
        let mut best_f = usize::MAX;
        let mut best_position = None;

        if !current.dead_end {
            best_f = current.f;
            best_position = Some(node.clone());
        }

        for child in node.as_ref().borrow().childs.iter() {
            let res = TreeNode::best_valid_position_recursion(child);
            if res.0 < best_f {
                best_f = res.0;
                best_position = res.1.clone();
            }
        }
        (best_f, best_position)
    }

    pub fn find_best_valid_position(node: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::best_valid_position_recursion(node).1
    }

    pub fn get_best_next_position(node: &Rc<RefCell<TreeNode>>, maze: &Maze) -> Option<Position> {
        let mut current = node.as_ref().borrow_mut();
        if current.dead_end {
            return None;
        }

        let current_access = maze
            .get_node_from_position(current.position)
            .unwrap()
            .access;

        // Get all unvisited positions
        let unvisited = maze.unvisited_neighbor(current.position);

        // Filter already visited positions (parent and childs of the node)
        let c = current.to_owned();
        let unvisited_possibilities = unvisited.iter().filter(|&e| {
            let d = c.position.get_direction(e).unwrap();
            current_access.contains(&d)
                && !c.childs.iter().any(|f| f.as_ref().borrow().position == *e)
                && !c.is_parent_position(e)
        });

        if unvisited_possibilities.clone().count() < 2 {
            current.dead_end = true;
        }

        // Get the best valid next position (the one with min(h) => min(f) # Because g the same for every child of this node)
        unvisited_possibilities
            .min_by_key(|&e| maze.get_distance(*e))
            .copied()
    }

    fn is_parent_position(&self, position: &Position) -> bool {
        let parent = &self.parent;
        let mut check_parent = false;
        if let Some(p) = parent {
            if let Some(p) = p.upgrade() {
                check_parent = p.as_ref().borrow().position == *position;
            }
        }
        check_parent
    }

    // Mark visited all the node from self to the root of the tree
    pub fn trace_path(&self, maze: &mut Maze) {
        let parent = &self.parent;
        if let Some(p) = parent {
            if let Some(p) = p.upgrade() {
                p.as_ref().borrow().trace_path(maze);
            }
        }
        maze.mark_position_visited(self.position);
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for ele in self.childs.iter() {
            writeln!(f, "{}", ele.as_ref().borrow())?
        }

        writeln!(f, "parent: {:#?}", self.parent)?;
        writeln!(f, "position: {:#?}", self.position)?;
        writeln!(f, "g: {}", self.g)?;
        writeln!(f, "h: {}", self.h)?;
        writeln!(f, "f: {}", self.f)?;

        write!(f, "")
    }
}

pub struct Tree {
    pub root: Option<TreeNodePtr>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, to_insert: &mut TreeNode) {
        match &mut self.root {
            Some(root) => TreeNode::insert_node(root, to_insert),
            None => self.root = Some(Rc::new(RefCell::new(to_insert.to_owned()))),
        }
    }

    pub fn find_best_valid_position(&self) -> Option<Rc<RefCell<TreeNode>>> {
        match &self.root {
            Some(root) => TreeNode::find_best_valid_position(root),
            None => None,
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.root {
            Some(root) => {
                writeln!(f, "{}", root.as_ref().borrow())
            }
            None => {
                write!(f, "None")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Tree, TreeNode};
    use crate::Maze;

    #[test]
    fn it_works() {
        let mut maze: Maze = Maze::new(10, 10);
        maze.make();
        println!("{maze}");

        let mut node = TreeNode::new(maze.find_start().unwrap());
        node.calculate_heuristic(&maze);

        let mut tree = Tree::new();
        tree.insert(&mut node);

        for _ in 0..20 {
            let mut find = tree.find_best_valid_position().unwrap();
            println!("Find node; {:#?}", find);

            if let Some(best_pos) = TreeNode::get_best_next_position(&find, &maze) {
                if maze.find_end().unwrap() == best_pos {
                    println!("FINISHED");
                    break;
                }
                maze.mark_position_visited(best_pos);
                let mut n = TreeNode::new(best_pos);
                n.calculate_heuristic(&maze);
                TreeNode::insert_node(&mut find, &mut n);
            }
        }

        println!("{maze}");
    }
}
