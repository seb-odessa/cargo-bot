use std::cell::Cell;
//use gridcell::CellId;
//use gridcell::GridCell;
//use gridcell::Neighbor;
use gridmap::GridMap;

#[allow(dead_code)] 
#[derive(Debug)]
pub struct GameBoard {
	id : usize,
	map : GridMap,
}
