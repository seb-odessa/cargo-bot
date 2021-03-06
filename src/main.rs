
extern crate lib;

use lib::maploader::Map;
use lib::gridcell::Way;
use lib::gameboard::GameBoard;


macro_rules! do_or_die {
        ($expr:expr) => (match $expr {
                    Ok(val) => val,
                    Err(why) => panic!("{}",why)
            })
}

#[allow(dead_code)]
fn main(){
    let map : Map = do_or_die!(Map::load("maps/flat_map_3x3.json"));

    let mut board : GameBoard = GameBoard::load(&map);

    let route = vec![Way::South, Way::South, Way::East, Way::East];

    println!("board.is_complete() => {:?}", board.run(route));

    println!("{}", board);
}
