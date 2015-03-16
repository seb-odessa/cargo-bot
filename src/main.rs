#![feature(box_syntax)]

extern crate lib;

use lib::gameboard::GameBoard;
use lib::maploader::Map;


#[allow(dead_code)]
fn main(){
	// let map : Map = Map::load_map("maps/flat_map_3x3.json");
	let map : Map = Map::demo_map();
 	let board : GameBoard = GameBoard::load(&map, 1, 9);
 	println!("board.is_complete() => {:?}", board.is_complete());


}
