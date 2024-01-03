use crate::Maze;

use super::Position;
use std::fmt::Display;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type NodePtr = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub parent: Option<Weak<RefCell<TreeNode>>>,
    pub childs: Vec<NodePtr>,
    pub position: Position,
    pub dead_end: bool,
    pub f: usize,
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
    pub fn insert_node(node: &mut NodePtr, to_insert: &mut TreeNode) {
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
    ) -> (usize, Rc<RefCell<TreeNode>>) {
        let mut best_f = node.as_ref().borrow().f;
        let mut best_position = node.clone();

        for child in node
            .as_ref()
            .borrow()
            .childs
            .iter()
            .filter(|&e| e.as_ref().borrow().dead_end == false)
        {
            let res = TreeNode::best_valid_position_recursion(child);
            println!("Values: res = {}, best_f = {best_f}", res.0);
            if res.0 < best_f {
                println!("Find lower");
                best_f = res.0;
                best_position = res.1.clone();
            }
        }
        (best_f, best_position)
    }

    pub fn find_best_valid_position(node: &Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        TreeNode::best_valid_position_recursion(node).1
    }

    pub fn get_best_next_position(node: &Rc<RefCell<TreeNode>>, maze: &Maze) -> Option<Position> {
        let current = node.as_ref().borrow().position;
        let current_access = maze.get_node_from_position(current).unwrap().access;
        let unvisited = maze.unvisited_neighbor(current);
        unvisited
            .iter()
            .filter(|&e| {
                let d = current.get_direction(e).unwrap();
                current_access.contains(&d)
            })
            .min_by_key(|&e| maze.get_distance(*e))
            .copied()
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

impl From<TreeNode> for Option<NodePtr> {
    fn from(node: TreeNode) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

pub struct Tree {
    pub root: Option<NodePtr>,
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
            Some(root) => Some(TreeNode::find_best_valid_position(root)),
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
    use crate::Position;

    #[test]
    fn it_works() {
        let mut maze: Maze = Maze::new(10, 10);
        maze.make();
        println!("{maze}");

        let mut node = TreeNode::new(Position { x: 1, y: 1 });
        node.calculate_heuristic(&maze);
        let mut node2 = TreeNode::new(Position { x: 2, y: 2 });
        node2.calculate_heuristic(&maze);
        let mut node3 = TreeNode::new(Position { x: 3, y: 2 });
        node3.calculate_heuristic(&maze);

        let mut tree = Tree::new();
        tree.insert(&mut node);
        tree.insert(&mut node2);
        tree.insert(&mut node3);

        //println!("{}", tree);

        let find = tree.find_best_valid_position().unwrap();
        println!("Find node; {:#?}", find);

        let best_pos = TreeNode::get_best_next_position(&find, &maze).unwrap();
        maze.mark_position_visited(best_pos);

        println!("{maze}");
    }
}
