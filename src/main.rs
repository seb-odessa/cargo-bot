
extern crate lib;

use lib::maploader::Map;
use lib::gridcell::Neighbor;
use lib::gameboard::GameBoard;
use lib::gameboard::Command;

macro_rules! do_or_die {
        ($expr:expr) => (match $expr {
                    Ok(val) => val,
                    Err(why) => panic!("{}",why)
            })
}

#[allow(dead_code)]
fn main(){
    let map : Map = do_or_die!(Map::load("maps/flat_map_3x3.json"));

    let mut game : GameBoard = GameBoard::load(&map, 1, 9);

    let route = vec![Neighbor::South,Neighbor::South,Neighbor::East,Neighbor::East];

    println!("board.is_complete() => {:?}", game.play(route));

    do_or_die!(Map::save("/tmp/the_map.json", &map));

    println!("{:?}", game);
}
