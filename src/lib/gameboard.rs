use maploader::Map;
use maploader::Cell;
use gridcell::CellId;
use gridmap::GridMap;

#[allow(dead_code)] 
#[derive(Debug)]
pub struct GameBoard {	
	map    : GridMap,
	cargo  : CellId,
	target : CellId,
	steps  : usize,
}

impl GameBoard {

	#[allow(dead_code)] 
	pub fn load(map : &Map, begin : CellId, target : CellId) -> GameBoard {
		GameBoard { map : GridMap::load(map),  cargo : begin, target : target, steps : 0 } 
	}

	#[allow(dead_code)] 
	pub fn is_complete(&self) -> bool {
		self.cargo == self.target && self.cargo != 0
	}


}

#[cfg(test)]
mod tests {
	use maploader::Map;
	use gameboard::GameBoard;

    #[test] 
    pub fn load()
    {
        let board = GameBoard::load(&Map::demo_map(), 1, 9);
        assert![board.cargo == 1];
        assert![board.target == 9];
    }

    #[test] 
    pub fn is_complete() {
    	let board = GameBoard::load(&Map::demo_map(), 1, 1);
    	assert![board.is_complete()];
    }

    #[test] 
    pub fn is_not_complete() {
    	let board = GameBoard::load(&Map::demo_map(), 1, 9);
    	assert![!board.is_complete()];
    }
}