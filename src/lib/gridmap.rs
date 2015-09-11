use std::collections::HashMap;
use gridcell::{Id, GridCell, Way};
use maploader::Map;
use std::fmt;


#[allow(dead_code)] 
#[derive(Debug)]
pub struct GridMap {
	pub cells : HashMap<Id, GridCell>,
}
impl fmt::Display for GridMap {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	for (id, cell) in self.cells.iter() {
	    try!(writeln!(f, "{:4}{}", id, cell));
	}
	write!(f, "")

    }
}
impl GridMap {
	#[allow(dead_code)] 
	pub fn new() -> GridMap {
		GridMap { cells : HashMap::new() } 
	}
	
	#[allow(dead_code)]
	pub fn exist(&self, id : Id) -> bool {
		self.cells.contains_key(&id)
	}

	#[allow(dead_code)]
	pub fn find(&self, id : Id) -> Option<&GridCell> {
		match self.cells.get(&id) {
			Some(cell) => Some(cell),
			None => None,
		}
	}

	#[allow(dead_code)]
	fn add_cell(&mut self, cell : GridCell) {
		self.ensure_new(cell.id);
		self.cells.insert(cell.id, cell);
	}

	#[allow(dead_code)]
	fn add_way(&mut self, lhv : Id, rhv : Id, way : Way) -> () {
		self.ensure_exist(lhv);
		self.ensure_exist(rhv);
		self.cells.get_mut(&lhv).unwrap().add(way, rhv);
	}

	#[allow(dead_code)]
	fn ensure_exist(&self, id : Id) -> () {
		assert!(self.exist(id));
	}

	#[allow(dead_code)]
	fn ensure_new(&self, id : Id) -> () {
		assert!(!self.exist(id));
	}

	#[allow(dead_code)]
	pub fn print_map(&self){
		for (key, cell) in self.cells.iter() {
				println!("{:2}: {}\n", key, cell);
		}
	}

	#[allow(dead_code)]
	pub fn load(map : &Map) -> GridMap
	{
	    let mut grid = GridMap::new();

	    for cell in map.cells.iter() {
	        grid.add_cell(GridCell::new(cell.id));
	    }
      
	    for cell in map.cells.iter() {
		    if cell.north > 0 {
			    grid.add_way(cell.id, cell.north, Way::North);
    		}
    		if cell.south > 0 {
	    		grid.add_way(cell.id, cell.south, Way::South);
    		}
    		if cell.west > 0 {
    			grid.add_way(cell.id, cell.west, Way::West);
    		}
    		if cell.east > 0 {
    			grid.add_way(cell.id, cell.east, Way::East);
    		}
	    }
      
	    return grid;
    }
}

#[cfg(test)]
mod tests {
	use gridcell::{GridCell, Way};
	use gridmap::GridMap;

	#[test] 
	pub fn new() {
		let grid = GridMap::new();
		assert!(grid.cells.is_empty());
	}

	#[test] 
	pub fn add_cell() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		assert_eq!(grid.cells.len(), 1);
	}

	#[test]
	pub fn exist() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		assert![grid.exist(1)];
	}

	#[test]
	pub fn find_existing() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		assert![grid.find(1).is_some()];
	}

	#[test]
	pub fn find_absent() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		assert![grid.find(2).is_none()];
	}

	#[test]
	pub fn ensure_exist() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		grid.ensure_exist(1);
	}

	#[test]
	pub fn ensure_new() {
		GridMap::new().ensure_new(1);
	}

	#[test]
	pub fn add_north() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		grid.add_cell(GridCell::new(2));
		grid.add_way(1, 2, Way::North);
		assert_eq!(grid.cells.get(&1).unwrap().get(Way::North).unwrap(), 2);
	}

	#[test]
	pub fn add_south() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		grid.add_cell(GridCell::new(2));
		grid.add_way(1, 2, Way::South);
		assert_eq!(grid.cells.get(&1).unwrap().get(Way::South).unwrap(), 2);
	}

	#[test]
	pub fn add_east() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		grid.add_cell(GridCell::new(2));
		grid.add_way(1, 2, Way::East);
		assert_eq!(grid.cells.get(&1).unwrap().get(Way::East).unwrap(), 2);
	}

	#[test]
	pub fn add_west() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		grid.add_cell(GridCell::new(2));
		grid.add_way(1, 2, Way::West);
		assert_eq!(grid.cells.get(&1).unwrap().get(Way::West).unwrap(), 2);
	}
}

#[cfg(test)]
mod panic {
	use gridcell::GridCell;
	use gridmap::GridMap;

	#[test]
	#[should_panic]
	pub fn ensure_exist() {
		GridMap::new().ensure_exist(1);
	}

	#[test]
	#[should_panic]
	pub fn ensure_new() {
		let mut grid = GridMap::new();
		grid.add_cell(GridCell::new(1));
		grid.ensure_new(1);
	}
}


