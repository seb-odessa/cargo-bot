#![feature(box_syntax)]
#![feature(alloc)]

use std::rc::Rc;

extern crate lib;

use lib::gridcell::GridCell;

#[allow(dead_code)]
fn main() {
	let cell1 = Rc::new(GridCell::new(1));
	let cell2 = Rc::new(GridCell::new(2));

	{
		cell1.link_north(cell2.downgrade());
		cell2.link_south(cell1.downgrade());
	}
	{
		println!("{}", cell1);
		println!("{}", cell2);
	}
	{
		let cell3 = cell1.get_north();
		println!("{}", cell3.unwrap());
	}
	

 }
