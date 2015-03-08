use std::cell::RefCell;
use std::fmt;

pub type CellId = usize;

#[allow(dead_code)] 
#[derive(Debug)]	
pub struct GridCell	{
	pub id			: CellId,
	pub north		: RefCell<Option<CellId>>,
	pub south		: RefCell<Option<CellId>>,
	pub east		: RefCell<Option<CellId>>,
	pub west		: RefCell<Option<CellId>>,
}

macro_rules! fmt_id{
    ($expr:expr) => (
    	match $expr {
    		Some(ref cell) => format!("{}", cell), 
    		None => "None".to_string() 
    	}
    )
}

impl fmt::Display for GridCell {
	#[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let north = fmt_id![*self.north.borrow()];
    	let south = fmt_id![*self.south.borrow()];
    	let east  = fmt_id![*self.east.borrow()];
    	let west  = fmt_id![*self.west.borrow()];
    	write!(f, "({}, north=>{}, south=>{}, east=>{}, west=>{})", self.id, north, south, east, west)
    }
}

impl GridCell {
	#[allow(dead_code)]
    pub fn new(id : CellId) -> GridCell {
    	GridCell { 
    		id    : id,
    		north : RefCell::new(None),
    		south : RefCell::new(None),
    		east  : RefCell::new(None),
    		west  : RefCell::new(None),
    	} 
    }

	#[allow(dead_code)]
    pub fn link_north(&self, rhv: &GridCell) -> () { (*self.north.borrow_mut()) = Some(rhv.id); }
	#[allow(dead_code)]
    pub fn link_south(&self, rhv: &GridCell) -> () { (*self.south.borrow_mut()) = Some(rhv.id); }
	#[allow(dead_code)]
    pub fn link_east(&self, rhv: &GridCell) -> () { (*self.east.borrow_mut()) = Some(rhv.id); }
	#[allow(dead_code)]
    pub fn link_west(&self, rhv: &GridCell) -> () { (*self.west.borrow_mut()) = Some(rhv.id); }

	#[allow(dead_code)]
    pub fn get_north(&self) -> Option<CellId> { *self.north.borrow() }
	#[allow(dead_code)]
    pub fn get_south(&self) -> Option<CellId> { *self.south.borrow() }
	#[allow(dead_code)]
    pub fn get_east(&self) -> Option<CellId> { *self.east.borrow() }
	#[allow(dead_code)]
    pub fn get_west(&self) -> Option<CellId> { *self.west.borrow() }
}

#[cfg(test)]
mod tests {
    #[test] 
    pub fn new()
    {
        let cell = ::GridCell::new(1);
        assert!(1 == cell.id);
        assert!((*cell.north.borrow()).is_none());
        assert!((*cell.south.borrow()).is_none());
        assert!((*cell.east.borrow()).is_none());
        assert!((*cell.west.borrow()).is_none());   
    }

    #[test] #[allow(dead_code)]
    pub fn link_north()
    {
        let cell = ::GridCell::new(1);
        cell.link_north(&::GridCell::new(2));
        assert![(*cell.north.borrow()).unwrap() == 2];
    } 
    #[test] #[allow(dead_code)]
    pub fn link_south()
    {
        let cell = ::GridCell::new(1);
        cell.link_south(&::GridCell::new(2));
        assert![(*cell.south.borrow()).unwrap() == 2];
    }
    #[test] #[allow(dead_code)]
    pub fn link_east()
    {
        let cell = ::GridCell::new(1);
        cell.link_east(&::GridCell::new(2));
        assert![(*cell.east.borrow()).unwrap() == 2];
    }
    #[test] #[allow(dead_code)]
    pub fn link_west()
    {
        let cell = ::GridCell::new(1);
        cell.link_west(&::GridCell::new(2));
        assert![(*cell.west.borrow()).unwrap() == 2];
    }
}


