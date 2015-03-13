#![feature(box_syntax)]

extern crate lib;

use lib::gridmap::GridMap;
use lib::maploader::Map;

#[allow(dead_code)]
fn main(){
	let map : Map = Map::load_map("maps/flat_map_3x3.json");
 	let grid : GridMap = GridMap::new();
 	grid.load_map(&map);
 	grid.print_map();
}
