#![feature(box_syntax)]
//#![feature(alloc)]

use std::rc::Rc;
use lib::gridcell::GridCell;
use lib::gridmap::GridMap;

extern crate lib;

#[allow(dead_code)]
fn main() {
	let cell1 = Rc::new(GridCell::new(1));
	let cell2 = Rc::new(GridCell::new(2));

	{
		cell1.link_north(&*cell2);
		cell2.link_south(&*cell1);
//		println!("{}", cell1);
//		println!("{}", cell2);
	}

	let grid = GridMap::new();
	grid.add_cell(cell1);
	grid.add_cell(cell2);
	grid.link_cells(1,2);
//	println!("cell1: {}", *grid.map.borrow().get(&1).unwrap());

}
