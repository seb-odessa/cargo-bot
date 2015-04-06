use maploader::Map;
use gridcell::CellId;
use gridmap::GridMap;
use gridcell::Neighbor;
use std::cell::Cell;

#[derive(Debug)]
pub enum Command { 
	Move,
	Stop,
}


#[allow(dead_code)] 
#[derive(Debug)]
pub struct GameBoard {	
	map    : GridMap,
	cargo  : Cell<CellId>,
	target : CellId,
	steps  : usize,
}

impl GameBoard {

	#[allow(dead_code)] 
	pub fn load(map : &Map, begin : CellId, target : CellId) -> GameBoard {
		GameBoard { map : GridMap::load(map),  cargo : Cell::new(begin), target : target, steps : 0 } 
	}

	#[allow(dead_code)] 
	pub fn is_complete(&self) -> bool {
		self.cargo.get() == self.target
	}

	#[allow(dead_code)] 
	pub fn execute(&self, cmd : Command, arg : Neighbor) -> bool {
		match cmd {
			Command::Move => self.do_move(arg),
			Command::Stop => false,
		}		
	}

	#[allow(dead_code)] 
	fn do_move(&self, direction : Neighbor) -> bool {
		let cargo = self.map.find(self.cargo.get());
		if cargo.is_none() { return false; }
		match cargo.unwrap().get_neighbor(direction) {
			Some(cell) => self.set_cargo(cell),
			None => false
		}
	}

	fn set_cargo(&self, cell : CellId) -> bool{
		self.cargo.set(cell);
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
        assert![board.cargo.get() == 1];
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