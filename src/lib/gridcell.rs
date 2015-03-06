use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::fmt;

#[allow(dead_code)] 
#[derive(Debug)]	
pub struct GridCell	{
	pub id			: usize,
	pub north		: RefCell<Option<Weak<GridCell>>>,
	pub south		: RefCell<Option<Weak<GridCell>>>,
	pub east		: RefCell<Option<Weak<GridCell>>>,
	pub west		: RefCell<Option<Weak<GridCell>>>,
}

macro_rules! fmt_id{
    ($expr:expr) => (
    	match $expr {
    		Some(ref exist) => format!("{}", exist.upgrade().unwrap().id), 
    		None => "None".to_string() 
    	}
    )
}

macro_rules! get_link{
    ($expr:expr) => (
    	match $expr {
    		Some(ref weak) => weak.upgrade(),
    		None => None
    	}
    )
}

impl fmt::Display for GridCell {
	#[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let north = fmt_id![*self.north.borrow()];
    	let south = fmt_id![*self.north.borrow()];
    	let east  = fmt_id![*self.east.borrow()];
    	let west  = fmt_id![*self.west.borrow()];
    	write!(f, "({}, north=>{}, south=>{}, east=>{}, west=>{})", self.id, north, south, east, west)
    }
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
    pub fn link_north(&self, rhv: Weak<GridCell>) -> () { (*self.north.borrow_mut()) = Some(rhv); }

	#[allow(dead_code)]
    pub fn link_south(&self, rhv: Weak<GridCell>) -> () { (*self.south.borrow_mut()) = Some(rhv); }

	#[allow(dead_code)]
    pub fn link_east(&self, rhv: Weak<GridCell>) -> () {  (*self.east.borrow_mut()) = Some(rhv);  }

	#[allow(dead_code)]
    pub fn link_west(&self, rhv: Weak<GridCell>) -> () {  (*self.west.borrow_mut()) = Some(rhv);  }

	#[allow(dead_code)]
    pub fn get_north(&self) -> Option<Rc<GridCell>> { get_link![*self.north.borrow()] }

	#[allow(dead_code)]
    pub fn get_south(&self) -> Option<Rc<GridCell>> { get_link![*self.south.borrow()] }

	#[allow(dead_code)]
    pub fn get_east(&self) -> Option<Rc<GridCell>> { get_link![*self.east.borrow()] }

	#[allow(dead_code)]
    pub fn get_west(&self) -> Option<Rc<GridCell>> { get_link![*self.west.borrow()] }
}
