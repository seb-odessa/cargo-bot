use maploader::Map;
use gridcell::CellId;
use gridmap::GridMap;
use gridcell::Neighbor;

#[derive(Debug)]
pub enum Command { 
	Move, 
}


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

	#[allow(dead_code)] 
	pub fn execute(&self, cmd : Command, arg : Neighbor) -> bool {
		match cmd {
			Move => self.do_move(arg),
		}		
	}

	#[allow(dead_code)] 
	fn do_move(&self, direction : Neighbor) -> bool {
		let cargo = self.map.find(self.cargo);
		if cargo.is_none() { return false; }
		if !cargo.unwrap().is_neighbor(direction) { return false; }

		true
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