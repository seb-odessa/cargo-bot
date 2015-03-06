#![feature(box_syntax)]
#![feature(alloc)]

use std::rc::Rc;

mod backend {
	use std::rc::Rc;
	use std::rc::Weak;
	use std::cell::RefCell;

	#[allow(dead_code)] 
	#[derive(Debug)]	
	pub struct GridCell	{
		pub id			: usize,
		pub north		: RefCell<Option<Weak<GridCell>>>,
		pub south		: RefCell<Option<Weak<GridCell>>>,
		pub east		: RefCell<Option<Weak<GridCell>>>,
		pub west		: RefCell<Option<Weak<GridCell>>>,
	}
	impl GridCell {
		#[allow(dead_code)]
	    pub fn new(id:usize) -> GridCell {
	    	GridCell { 
	    		id : id,
	    		north : RefCell::new(None),
	    		south : RefCell::new(None),
	    		east  : RefCell::new(None),
	    		west  : RefCell::new(None),
	    	} 
	    }

   		#[allow(dead_code)]
	    pub fn link_north(&self, rhv: Weak<GridCell>) -> () {
            (*self.north.borrow_mut()) = Some(rhv);
        }

   		#[allow(dead_code)]
	    pub fn link_south(&self, rhv: Weak<GridCell>) -> () {
            (*self.south.borrow_mut()) = Some(rhv);
        }

   		#[allow(dead_code)]
	    pub fn link_east(&self, rhv: Weak<GridCell>) -> () {
            (*self.east.borrow_mut()) = Some(rhv);
        }
   		#[allow(dead_code)]
	    pub fn link_west(&self, rhv: Weak<GridCell>) -> () {
            (*self.west.borrow_mut()) = Some(rhv);
        }

   		#[allow(dead_code)]
	    pub fn get_north(&self) -> Option<Rc<GridCell>> {
            match *self.north.borrow() {
	    		Some(ref weak) => weak.upgrade(),
	    		None => return None
	    	}
        }
	}
}

fn main() {
    let cell1 = Rc::new(backend::GridCell::new(1));
    let cell2 = Rc::new(backend::GridCell::new(2));
    let cell3 = Rc::new(backend::GridCell::new(3));
        
    {
	  	println!("{:?}", cell1);
  		println!("{:?}", cell2);
	  	println!("{:?}", cell3);
    }
 
  	{
//  		cell1.set_link(cell2.downgrade());
//  		cell2.set_link(cell3.downgrade());
  	}

    {
	  	println!("{:?}", cell1);
  		println!("{:?}", cell2);
  		println!("{:?}", cell3);
    } 	 

	{
	    // println!("cell1.get_link() => {:?}", cell1.get_link());
	    // println!("cell2.get_link() => {:?}", cell2.get_link());
	    // println!("cell3.get_link() => {:?}", cell3.get_link());
	}
 }
