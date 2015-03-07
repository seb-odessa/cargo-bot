#![feature(box_syntax)]
#![feature(alloc)]

use std::rc::Rc;
use lib::gridcell::GridCell;

extern crate lib;

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
// 	{
//  use std::sync::TaskPool;
// 	use std::iter::AdditiveIterator;
// 	use std::old_io::process::Command;
// 		let pool = TaskPool::new(8);
// 		for i in range(0, 1000)		
// 		{
// 		    pool.execute(move|| {println!("{}", Gen::get());});
// 		}
// 		let _process = Command::new("sleep").arg("2").spawn();	    
// 	}
}
