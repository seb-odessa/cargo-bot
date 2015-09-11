//! This library contains representation of the linked wetwork of cells.
//!
//! The possible way for traverse of the network
//!
//! For example:
//!
//! ```
//! extern crate lib;
//! 
//! use lib::maploader::Map;
//! use lib::gridcell::Way;
//! use lib::gameboard::GameBoard;
//! 
//! #[allow(dead_code)]
//! fn main(){
//!     let map : Map = Map::demo_map();
//!     let mut board : GameBoard = GameBoard::load(&map);
//!
//!     let route = vec![Way::South, Way::South, Way::East, Way::East];
//!     for way in &route {
//!         println!("board.goto({:?}) => {}", *way, board.goto(way));
//!     }
//!     println!("board.is_complete() => {:?}", board.is_complete());
//! }
//! ```
//!
//! will produce:
//! 
//! board.goto(South) => true
//!
//! board.goto(South) => true
//!
//! board.goto(East) => true
//!
//! board.goto(East) => true 
//!
//! board.is_complete() => true
//!
//! 


pub use self::gridcell::{Id, GridCell, Way};
pub use self::gridmap::GridMap;
pub use self::maploader::Map;
pub use self::gameboard::GameBoard;

pub mod gridcell;
pub mod gridmap;
pub mod maploader;
pub mod gameboard;

extern crate rustc_serialize;


