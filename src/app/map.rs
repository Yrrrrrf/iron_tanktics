//! Represents the game map and its contents.
//! It is a grid of tiles, each of which can contain a single entity.
//!
//! It has a width and height, and a list of entities. The entities are stored in a separate list to allow for fast iteration over them.
//! The map is responsible for updating the entities, and for rendering itself.

// use crate::Tank;
// use bevy::prelude::*;


// /// Represents a single tile on the map.
// #[derive(Clone, Debug)]
// pub enum Tile {
//     Empty,
//     Wall,
//     Tank(Tank),
// }


// /// Represents the game map.
// /// It is a grid of tiles, each of which can contain a single entity.
// /// 
// /// It has a width and height, and a list of entities. The entities are stored in a separate list to allow for fast iteration over them.
// #[derive(Clone, Debug)]
// pub struct Map {
//     width: u8,
//     height: u8,
//     tiles: Vec<Tile>,
// }

// impl Map {
//     /// Create a new map with the given width and height.
//     pub fn new(width: u8, height: u8) -> Map {
//         Map {
//             width,
//             height,
//             tiles: vec![Tile::Empty; (width * height) as usize],
//         }
//     }

//     // /// Get the width of the map.
//     // pub fn width(&self) -> u8 {
//     //     self.width
//     // }

//     // /// Get the height of the map.
//     // pub fn height(&self) -> u8 {
//     //     self.height
//     // }

//     /// Get the tile at the given position.
//     pub fn get_tile(&self, x: u8, y: u8) -> &Tile {
//         &self.tiles[(y * self.width + x) as usize]
//     }

//     /// Get the tile at the given position.
//     pub fn get_tile_mut(&mut self, x: u8, y: u8) -> &mut Tile {
//         &mut self.tiles[(y * self.width + x) as usize]
//     }

//     /// Set the tile at the given position.
//     pub fn set_tile(&mut self, x: u8, y: u8, tile: Tile) {
//         self.tiles[(y * self.width + x) as usize] = tile;
//     }

//     /// Update the map and all of its entities.
//     pub fn update(&mut self) {
//         for tile in &mut self.tiles {
//             match tile {
//                 Tile::Tank(tank) => {
//                     if tank.is_dead() {
//                         *tile = Tile::Empty;
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }
// }




