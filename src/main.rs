#![feature(box_syntax)]

extern crate lib;


use lib::maploader::Map;
use lib::gridcell::Neighbor;
use lib::gameboard::GameBoard;
use lib::gameboard::Command;


#[allow(dead_code)]
fn main(){
	// let map : Map = Map::load_map("maps/flat_map_3x3.json");
	let map : Map = Map::demo_map();
 	let board : GameBoard = GameBoard::load(&map, 1, 9);
 	

 	let route = vec![Neighbor::South,Neighbor::South,Neighbor::South,Neighbor::East,Neighbor::East,Neighbor::East,];

 	for direction in route.iter() {
        println!("direction: {:?}, succeeded: {}", direction, board.execute(Command::Move, direction));

        println!("board.is_complete() => {:?}", board.is_complete());
    }


}
