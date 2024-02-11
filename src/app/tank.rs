//! This file defines the `Tank` struct and its methods.


#[derive(Debug, Clone)]
pub struct Tank {
    life: u8,
    // fuel: u8,
    // ammo: u8,
}

impl Tank {
    pub fn new() -> Tank {
        Tank {
            life: 100,
            // fuel: 100,
            // ammo: 100,
        }
    }

    pub fn life(&self) -> u8 {
        self.life
    }

    pub fn is_dead(&self) -> bool {
        self.life == 0
    }

    fn take_damage(&mut self, damage: u8) {
        self.life = if self.life > damage { self.life - damage } else { 0 };
    }
}


// create the action trait for the tank
pub trait TankAction {
    // Attaks
    fn fire_missile(&mut self, direction: MovDir);
    fn fire_mortar(&mut self, target: (u8, u8));

    // Movement
    fn r#move(&mut self, direction: MovDir);

    // Query
    fn radar(&self, direction: MovDir) -> Vec<(u8, bool)>;
}


#[derive(Debug)]
// create the movement direction enum
pub enum MovDir {
    Up,
    Down,
    Left,
    Right,
}


impl TankAction for Tank {
    fn fire_missile(&mut self, direction: MovDir) {
        todo!("Firing missile! {:?}", direction);
    }

    fn fire_mortar(&mut self, target: (u8, u8)) {
        todo!("Firing mortar at {:?}", target);
    }


    fn r#move(&mut self, direction: MovDir) {
        match direction {
            MovDir::Up => println!("Moving up!"),
            MovDir::Down => println!("Moving down!"),
            MovDir::Left => println!("Moving left!"),
            MovDir::Right => println!("Moving right!"),
        }
    }


    fn radar(&self, direction: MovDir) -> Vec<(u8, bool)> {
        todo!("Radar scan in direction {:?}", direction);
    }

}