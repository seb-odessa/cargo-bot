
extern crate lib;

use lib::maploader::Map;
use lib::gridcell::Neighbor;
use lib::gameboard::GameBoard;
use lib::gameboard::Command;

#[allow(dead_code)]
fn main(){
    let map : Map = match Map::load("maps/flat_map_3x3.json") {
        Err(why) => panic!("ERROR: {}", why),
        Ok(map) => map
    };

//    let map : Map = Map::demo_map();
    let board : GameBoard = GameBoard::load(&map, 1, 9);

    let route = vec![Neighbor::South,Neighbor::South,Neighbor::East,Neighbor::East,];
    for direction in route.iter() {
        println!("board.execute(Command::Move, {:?}) => {}", *direction, board.execute(Command::Move, *direction));
        println!("board.is_complete() => {:?}", board.is_complete());
    }

    match Map::save("maps/new_map.json", &map) {
        Err(why) => panic!("ERROR: {}", why),
        Ok(_) => ()
    }

    println!("{:?}", board);
}
