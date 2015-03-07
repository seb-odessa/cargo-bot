use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;
use GridCell;

#[allow(dead_code)] 
#[derive(Debug)]	
pub struct GridMap	{
	pub map			: RefCell<BTreeMap<usize, Rc<GridCell>>>,
}
impl GridMap {
	#[allow(dead_code)] 
	pub fn new() -> GridMap {
		GridMap { map : RefCell::new(BTreeMap::new()) } 
	}

	#[allow(dead_code)]
	pub fn add_cell(&self, cell : Rc<GridCell>) {
		assert_eq!(self.map.borrow().contains_key(&cell.id), false);
		self.map.borrow_mut().insert(cell.id, cell);		
	}

	#[allow(dead_code)]
	pub fn link_cells(&self, lhv : usize, rhv : usize) -> () {
		assert_eq!(self.map.borrow().contains_key(&lhv), true);
		assert_eq!(self.map.borrow().contains_key(&rhv), true);			
		self.map.borrow().get(&lhv).unwrap().link_north(self.map.borrow().get(&rhv).unwrap().downgrade());
		self.map.borrow().get(&rhv).unwrap().link_south(self.map.borrow().get(&lhv).unwrap().downgrade());
	}
}

#[test] #[allow(dead_code)]
pub fn new() {
	let grid = GridMap::new();
	assert!(grid.map.borrow().is_empty());
}

#[test] #[allow(dead_code)] #[should_fail]
pub fn add_cell_not_unique() {
	let grid = GridMap::new();
	grid.add_cell(Rc::new(GridCell::new(1)));
	grid.add_cell(Rc::new(GridCell::new(1)));
}
#[test] #[allow(dead_code)]
pub fn add_cell() {
	let grid = GridMap::new();
	grid.add_cell(Rc::new(GridCell::new(1)));
	assert_eq!(grid.map.borrow().len(), 1);
}

#[test] #[allow(dead_code)] #[should_fail]
pub fn link_cells_lhv_absent() {
	let grid = GridMap::new();
	grid.add_cell(Rc::new(GridCell::new(1)));
	grid.link_cells(0, 1);
}
#[test] #[allow(dead_code)] #[should_fail]
pub fn link_cells_rhv_absent() {
	let grid = GridMap::new();
	grid.add_cell(Rc::new(GridCell::new(1)));
	grid.link_cells(1, 0);
}
#[test] #[allow(dead_code)]
pub fn link_cells() {
	let grid = GridMap::new();
	grid.add_cell(Rc::new(GridCell::new(1)));
	grid.add_cell(Rc::new(GridCell::new(2)));
	grid.link_cells(1,2);
	assert_eq!(grid.map.borrow().get(&1).unwrap().get_north().unwrap().id, 2);
	assert_eq!(grid.map.borrow().get(&2).unwrap().get_south().unwrap().id, 1);
}
