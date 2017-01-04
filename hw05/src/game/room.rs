use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {
    // TODO: Implement the necessary methods for Rooms.

    pub fn new() -> Room {
        Room {
            name: "haha".to_string(),
            contents: vec![],
            halls: vec![],
            wumpus: false
        }
    }
    pub fn neighbors_string(&self) -> String {
        // TODO: Implement
        unimplemented!();
    }

    pub fn neighbors(&self) -> Vec<Rc<RefCell<Room>>> {
        let mut neighbors = vec![];

        for hall in self.halls.clone() {
            let left = hall.left.borrow();
            let right = hall.right.borrow();

            if *self == *left {
                neighbors.push(hall.right.clone());
            } else if *self == *right {
                neighbors.push(hall.left.clone());
            }
        }

        neighbors
    }
    pub fn consum_contents(&mut self) -> Vec<Curio> {
        mem::replace(&mut self.contents, vec![])
    }
}
