use maploader::Map;
use gridcell::{Id, Neighbor};
use gridmap::GridMap;

#[derive(Debug)]
pub enum Command { 
	Move,
	Stop,
}


#[allow(dead_code)] 
#[derive(Debug)]
pub struct GameBoard {	
	map    : GridMap,
	current: Id,
	finish : Id,
	steps  : usize,
}

impl GameBoard {

	#[allow(dead_code)] 
	pub fn load(map : &Map, start : Id, finish : Id) -> GameBoard {
		GameBoard { map : GridMap::load(map), current : start, finish : finish, steps : 0 } 
	}

	#[allow(dead_code)] 
	pub fn is_complete(&self) -> bool {
		self.current == self.finish
	}

	#[allow(dead_code)] 
	pub fn execute(&mut self, cmd : Command, dir : Neighbor) -> bool {
		let is_ok = match cmd {
			Command::Move => self.do_move(dir),
			Command::Stop => false,
		};
        return is_ok && !self.is_complete()
	}

	#[allow(dead_code)] 
	fn do_move(&mut self, dir : Neighbor) -> bool {
        let curr_pos = self.map.find(self.current);
        if curr_pos.is_none() { 
            return false; 
        }
        let next = curr_pos.unwrap().get_neighbor(dir);
        if next.is_none() {
            return false;
        }
     
        self.current = next.unwrap(); 
        self.steps = self.steps + 1;
        return true;
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
        assert![board.current == 1];
        assert![board.finish == 9];
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

