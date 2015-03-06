#![feature(box_syntax)]
#![feature(alloc)]

use std::rc::Rc;

extern crate lib;

use lib::gridcell::GridCell;

fn main() {
    let cell1 = Rc::new(GridCell::new(1));
    let cell2 = Rc::new(GridCell::new(2));
    let cell3 = Rc::new(GridCell::new(3));
         
  	{
 		cell1.link_north(cell2.downgrade());
 		cell2.link_south(cell1.downgrade());

 		cell2.link_north(cell3.downgrade());
 		cell3.link_south(cell2.downgrade());
  	}

    {
	  	println!("{}", cell1);
  		println!("{}", cell2);
  		println!("{}", cell3);
    } 	 
 }
