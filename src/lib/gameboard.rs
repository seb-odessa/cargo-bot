use maploader::Map;
use gridcell::{Id, Way};
use gridmap::GridMap;
use std::fmt;

pub type Route = Vec<Way>;

#[allow(dead_code)] 
#[derive(Debug)]
pub struct GameBoard {
	map    : GridMap,
	current: Id,
	target : Id,
	steps  : usize,
}

impl fmt::Display for GameBoard {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}\n", self.map));
        try!(write!(f, "current : {}\n", self.current));
        try!(write!(f, "target : {}\n", self.target));
        write!(f, "steps : {}\n", self.steps)
    }
}

impl GameBoard {

	#[allow(dead_code)] 
	pub fn load(map : &Map) -> GameBoard {
		GameBoard { map : GridMap::load(map), current : map.start, target : map.target, steps : 0 } 
	}

	#[allow(dead_code)] 
	pub fn run(&mut self, route : Route) -> bool {
	    for way in &route {
            self.goto(way);
        }
        return self.is_complete()
	}

	#[allow(dead_code)] 
	pub fn is_complete(&self) -> bool {
		self.current == self.target
	}

	#[allow(dead_code)] 
	pub fn goto(&mut self, way : &Way) -> bool {
        if let Some(current) = self.map.find(self.current) {
            if let Some(next) = current.get(*way) {
                self.current = next;
                self.steps = self.steps + 1;
                return true;
            }
        }
        return false;
	}
}


#[cfg(test)]
mod tests {
	use maploader::Map;
	use gameboard::GameBoard;

    #[test] 
    pub fn load()
    {
        let board = GameBoard::load(&Map::demo_map());
        assert![board.current == 1];
        assert![board.target == 9];
    }

    #[test] 
    pub fn is_complete() {
    	let mut board = GameBoard::load(&Map::demo_map());
        board.current = board.target;
    	assert![board.is_complete()];
    }

    #[test] 
    pub fn is_not_complete() {
    	let board = GameBoard::load(&Map::demo_map());
    	assert![!board.is_complete()];
    }
}

