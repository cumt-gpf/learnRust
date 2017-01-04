use std;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::curio::Curio;
use super::room::Room;

const MAX_HP: i32 = 25;

pub enum Command {
    Go(String),
    Shoot(String),
}

pub struct Player {
    pub location: Rc<RefCell<Room>>,
    pub hp: i32,
    pub gold: i32,
    won: bool,
}

impl Player {
    pub fn new(location: Rc<RefCell<Room>>) -> Player {
        Player {
            location: location,
            hp: MAX_HP,
            gold: 0,
            won: false,
        }
    }

    pub fn use_curio(&mut self, curio: Curio) {
        match curio {
            Curio::Chest(gold) => {
                println!("You open the chest and gain {} gold.", gold);
                self.gold += gold;
            },
            Curio::SpikeTrap(dmg) => {
                println!("You take {} damage from the spikes.", dmg);
                self.hp -= dmg;
            },
            Curio::Food(heal) => {
                println!("You shove a wall chicken into your gob and heal {} HP.", heal);
                self.hp = std::cmp::min(MAX_HP, self.hp + heal);
            },
            Curio::IronMaiden(sub, dmg) => {
                println!("Dude I love Iron Maiden! This one's pointy, though.");
                println!("You cut yourself on the spikes inside for {} damage.", dmg);
                self.hp -= dmg;
                println!("You open the iron maiden and...");
                self.use_curio(*sub);
            },
            Curio::FallenAdventurer(sub) => {
                println!("You pilfer the corpse and...");
                self.use_curio(*sub);
            },
        }
    }

    /// Execute the given command on the player and board state.
    pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
        match cmd {
            Command::Go(des) => {
                let neighbor_room = try!(self.find_room(des));
                let mut room = neighbor_room.borrow_mut();
                println!("Welcome to room: {}", room.name);

                for curio in room.consum_contents() {
                    self.use_curio(curio);
                }

                if room.wumpus {
                    println!("Oops you are eaten by Wumpus!");
                    self.hp = 0;
                } 
                self.location = neighbor_room.clone();
                Ok(())
            },
            Command::Shoot(des) => {
                let neighbor_room = try!(self.find_room(des));

                let room = neighbor_room.borrow();

                if room.wumpus {
                    println!("You kill the wumpus");
                    self.won = true;
                } else {
                    println!("You miss one arrow");
                }

                Ok(())


            },
        }
    }

    /// Find one of the neighbors of the current room based on its name. Case insensitive.
    fn find_room(&self, room: String) -> Result<Rc<RefCell<Room>>, ()> {
        let current = self.location.clone();
        let current_room = current.borrow();

        current_room.neighbors().iter().find(|n| {
            let neighbor = n.borrow();
            neighbor.name.to_lowercase() == room.to_lowercase()
        })
            .map(|neighbor| neighbor.clone())
            .ok_or(())
    }
}

/**/impl fmt::Display for Player {
/**/    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
/**/        write!(f, "You find yourself in {}.\n\nYou have {} HP and {} gold.",
/**/               self.location.borrow().name, self.hp, self.gold)
/**/    }
/**/}
