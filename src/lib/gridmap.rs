use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

use gridcell::CellId;
use gridcell::GridCell;
use gridcell::Neighbor;
use maploader::Map;


#[allow(dead_code)] 
#[derive(Debug)]
pub struct GridMap	{
	pub map			: RefCell<BTreeMap<CellId, Rc<GridCell>>>,
}
impl GridMap {
	#[allow(dead_code)] 
	pub fn new() -> GridMap {
		GridMap { map : RefCell::new(BTreeMap::new()) } 
	}

	#[allow(dead_code)]
	fn add_cell(&self, cell : Rc<GridCell>) {
		self.ensure_new(cell.id);
		self.map.borrow_mut().insert(cell.id, cell);		
	}

	#[allow(dead_code)]
	fn add_neighbor(&self, lhv : CellId, rhv : CellId, neighbor : Neighbor) -> () {
		// link neighbor according to direction
		self.ensure_exist(lhv);
		self.ensure_exist(rhv);
		self.map.borrow().get(&lhv).unwrap().add_neighbor(neighbor, rhv);
	}

	#[allow(dead_code)]
	fn ensure_exist(&self, id : CellId) -> () {
		assert_eq!(self.map.borrow().contains_key(&id), true);
	}

	#[allow(dead_code)]
	fn ensure_new(&self, id : CellId) -> () {
		assert_eq!(self.map.borrow().contains_key(&id), false);
	}

	#[allow(dead_code)]
	pub fn print_map(&self){
		for (key, cell) in self.map.borrow().iter() {
				println!("{}: {}", key, cell);
		}
	}

	#[allow(dead_code)]
	pub fn load_map(&self, map : &Map)
	{
		self.map.borrow_mut().clear();
		// Add all cells to the map
		for cell in map.cells.iter() {
			self.add_cell(Rc::new(GridCell::new(cell.id)));
			println!("The cell {} was added to the map.", cell.id);
		}
		// Link all cells in the map
		for cell in map.cells.iter() {
			if cell.north > 0 {
				self.add_neighbor(cell.id, cell.north, Neighbor::North);
				println!("For the cell {} was added NORTH neighbor {}.", cell.id, cell.north);
			}
			if cell.south > 0 {
				self.add_neighbor(cell.id, cell.south, Neighbor::South);
				println!("For the cell {} was added SOUTH neighbor {}.", cell.id, cell.south);
			}
			if cell.west > 0 {
				self.add_neighbor(cell.id, cell.west, Neighbor::West);
				println!("For the cell {} was added WEST neighbor {}.", cell.id, cell.west);
			}
			if cell.east > 0 {
				self.add_neighbor(cell.id, cell.east, Neighbor::East);
				println!("For the cell {} was added EAST neighbor {}.", cell.id, cell.east);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;
	use gridcell::GridCell;
	use gridcell::Neighbor;
	use gridmap::GridMap;

	#[test] 
	pub fn new() {
		let grid = GridMap::new();
		assert!(grid.map.borrow().is_empty());
	}

	#[test] 
	pub fn add_cell() {
		let grid = GridMap::new();
		grid.add_cell(Rc::new(GridCell::new(1)));
		assert_eq!(grid.map.borrow().len(), 1);
	}

	#[test]
	pub fn ensure_exist() {
		let grid = GridMap::new();
		grid.add_cell(Rc::new(GridCell::new(1)));
		grid.ensure_exist(1);
	}

	#[test]
	pub fn ensure_new() {
		GridMap::new().ensure_new(1);
	}

	#[test]
	pub fn add_north_neighbor() {
		let grid = GridMap::new();
		grid.add_cell(Rc::new(GridCell::new(1)));
		grid.add_cell(Rc::new(GridCell::new(2)));
		grid.add_neighbor(1, 2, Neighbor::North);
		assert_eq!(grid.map.borrow().get(&1).unwrap().get_neighbor(Neighbor::North).unwrap(), 2);
	}

	#[test]
	pub fn add_south_neighbor() {
		let grid = GridMap::new();
		grid.add_cell(Rc::new(GridCell::new(1)));
		grid.add_cell(Rc::new(GridCell::new(2)));
		grid.add_neighbor(1, 2, Neighbor::South);
		assert_eq!(grid.map.borrow().get(&1).unwrap().get_neighbor(Neighbor::South).unwrap(), 2);
	}

	#[test]
	pub fn add_east_neighbor() {
		let grid = GridMap::new();
		grid.add_cell(Rc::new(GridCell::new(1)));
		grid.add_cell(Rc::new(GridCell::new(2)));
		grid.add_neighbor(1, 2, Neighbor::East);
		assert_eq!(grid.map.borrow().get(&1).unwrap().get_neighbor(Neighbor::East).unwrap(), 2);
	}

	#[test]
	pub fn add_west_neighbor() {
		let grid = GridMap::new();
		grid.add_cell(Rc::new(GridCell::new(1)));
		grid.add_cell(Rc::new(GridCell::new(2)));
		grid.add_neighbor(1, 2, Neighbor::West);
		assert_eq!(grid.map.borrow().get(&1).unwrap().get_neighbor(Neighbor::West).unwrap(), 2);
	}
}

#[cfg(test)]
mod panic {
	use std::rc::Rc;
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
		let grid = GridMap::new();
		grid.add_cell(Rc::new(GridCell::new(1)));
		grid.ensure_new(1);
	}
}