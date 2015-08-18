//use std::rc::Rc;
//use std::cell::RefCell;
use std::collections::HashMap;

use gridcell::{Id, GridCell, Neighbor};
use maploader::Map;


#[allow(dead_code)] 
#[derive(Debug)]
pub struct GridMap	{
	pub cells			: HashMap<Id, GridCell>,
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
	fn add(&mut self, cell : GridCell) {
		self.ensure_new(cell.id);
		self.cells.insert(cell.id, cell);		
	}

	#[allow(dead_code)]
	fn add_neighbor(&mut self, lhv : Id, rhv : Id, neighbor : Neighbor) -> () {
		// link neighbor according to direction
		self.ensure_exist(lhv);
		self.ensure_exist(rhv);
		self.cells.get_mut(&lhv).unwrap().add_neighbor(neighbor, rhv);
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
				println!("{:2}: {}", key, cell);
		}
	}

	#[allow(dead_code)]
	pub fn load(map : &Map) -> GridMap
	{
        let mut grid = GridMap::new();
		// Add all cells to the storage
		for cell in map.cells.iter() {
			grid.add(GridCell::new(cell.id));
		}
      
		// Link all cells in the map
		for cell in map.cells.iter() {
			if cell.north > 0 {
				grid.add_neighbor(cell.id, cell.north, Neighbor::North);
			}
			if cell.south > 0 {
				grid.add_neighbor(cell.id, cell.south, Neighbor::South);
			}
			if cell.west > 0 {
				grid.add_neighbor(cell.id, cell.west, Neighbor::West);
			}
			if cell.east > 0 {
				grid.add_neighbor(cell.id, cell.east, Neighbor::East);
			}
		}
      
	return grid;
    }
}

#[cfg(test)]
mod tests {
	use gridcell::{GridCell, Neighbor};
	use gridmap::GridMap;

	#[test] 
	pub fn new() {
		let grid = GridMap::new();
		assert!(grid.cells.is_empty());
	}

	#[test] 
	pub fn add_cell() {
		let mut grid = GridMap::new();
		grid.add(GridCell::new(1));
		assert_eq!(grid.cells.len(), 1);
	}

	#[test]
	pub fn exist() {
		let mut grid = GridMap::new();
		grid.add(GridCell::new(1));
		assert![grid.exist(1)];
	}

	#[test]
	pub fn find_existing() {
		let mut grid = GridMap::new();
		grid.add(GridCell::new(1));
		assert![grid.find(1).is_some()];
	}

	#[test]
	pub fn find_absent() {
		let mut grid = GridMap::new();
		grid.add(GridCell::new(1));
		assert![grid.find(2).is_none()];
	}

	#[test]
	pub fn ensure_exist() {
		let mut grid = GridMap::new();
		grid.add(GridCell::new(1));
		grid.ensure_exist(1);
	}

	#[test]
	pub fn ensure_new() {
		GridMap::new().ensure_new(1);
	}

	#[test]
	pub fn add_north_neighbor() {
		let mut grid = GridMap::new();
		grid.add(GridCell::new(1));
		grid.add(GridCell::new(2));
		grid.add_neighbor(1, 2, Neighbor::North);
		assert_eq!(grid.cells.get(&1).unwrap().get_neighbor(Neighbor::North).unwrap(), 2);
	}

	#[test]
	pub fn add_south_neighbor() {
		let mut grid = GridMap::new();
		grid.add(GridCell::new(1));
		grid.add(GridCell::new(2));
		grid.add_neighbor(1, 2, Neighbor::South);
		assert_eq!(grid.cells.get(&1).unwrap().get_neighbor(Neighbor::South).unwrap(), 2);
	}

	#[test]
	pub fn add_east_neighbor() {
		let mut grid = GridMap::new();
		grid.add(GridCell::new(1));
		grid.add(GridCell::new(2));
		grid.add_neighbor(1, 2, Neighbor::East);
		assert_eq!(grid.cells.get(&1).unwrap().get_neighbor(Neighbor::East).unwrap(), 2);
	}

	#[test]
	pub fn add_west_neighbor() {
		let mut grid = GridMap::new();
		grid.add(GridCell::new(1));
		grid.add(GridCell::new(2));
		grid.add_neighbor(1, 2, Neighbor::West);
		assert_eq!(grid.cells.get(&1).unwrap().get_neighbor(Neighbor::West).unwrap(), 2);
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
		grid.add(GridCell::new(1));
		grid.ensure_new(1);
	}
}


