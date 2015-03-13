use std::cell::RefCell;
use std::fmt;

pub type CellId = usize;

#[allow(dead_code)] 
#[derive(Debug)]
pub enum Neighbor{ North, South, East, West }

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
    pub fn add_neighbor(&self, dir: Neighbor, id: CellId) -> () { 
        match dir {
            Neighbor::North => (*self.north.borrow_mut()) = Some(id),
            Neighbor::South => (*self.south.borrow_mut()) = Some(id),
            Neighbor::East => (*self.east.borrow_mut()) = Some(id),
            Neighbor::West => (*self.west.borrow_mut()) = Some(id),
        }
    }

    #[allow(dead_code)]
    pub fn get_neighbor(&self, dir: Neighbor) -> Option<CellId> { 
        match dir {
            Neighbor::North => *self.north.borrow(),
            Neighbor::South => *self.south.borrow(),
            Neighbor::East => *self.east.borrow(),
            Neighbor::West => *self.west.borrow(),
        }
    }
}

#[cfg(test)]
mod tests {
    use gridcell::GridCell;
    use gridcell::Neighbor;
    #[test] 
    pub fn new()
    {
        let cell = GridCell::new(1);
        assert!(1 == cell.id);
        assert!((*cell.north.borrow()).is_none());
        assert!((*cell.south.borrow()).is_none());
        assert!((*cell.east.borrow()).is_none());
        assert!((*cell.west.borrow()).is_none());   
    }

    #[test] #[allow(dead_code)]
    pub fn add_neighbor_north()
    {
        let cell = GridCell::new(1);
        cell.add_neighbor(Neighbor::North, GridCell::new(2).id);
        assert![(*cell.north.borrow()).unwrap() == 2];
    } 

    #[test] #[allow(dead_code)]
    pub fn add_neighbor_south()
    {
        let cell = GridCell::new(1);
        cell.add_neighbor(Neighbor::South, GridCell::new(2).id);
        assert![(*cell.south.borrow()).unwrap() == 2];
    }

    #[test] #[allow(dead_code)]
    pub fn add_neighbor_east()
    {
        let cell = ::GridCell::new(1);
        cell.add_neighbor(Neighbor::East, GridCell::new(2).id);
        assert![(*cell.east.borrow()).unwrap() == 2];
    }

    #[test] #[allow(dead_code)]
    pub fn add_neighbor_west()
    {
        let cell = ::GridCell::new(1);
        cell.add_neighbor(Neighbor::West, GridCell::new(2).id);
        assert![(*cell.west.borrow()).unwrap() == 2];
    }

    #[test] #[allow(dead_code)]
    pub fn get_neighbor_north()
    {
        let cell = GridCell::new(1);
        (*cell.north.borrow_mut()) = Some(2);
        assert![cell.get_neighbor(Neighbor::North).unwrap() == 2];
    } 

    #[test] #[allow(dead_code)]
    pub fn get_neighbor_south()
    {
        let cell = GridCell::new(1);
        (*cell.south.borrow_mut()) = Some(2);
        assert![cell.get_neighbor(Neighbor::South).unwrap() == 2];
    }

    #[test] #[allow(dead_code)]
    pub fn get_neighbor_east()
    {
        let cell = ::GridCell::new(1);
        (*cell.east.borrow_mut()) = Some(2);
        assert![cell.get_neighbor(Neighbor::East).unwrap() == 2];
    }
    
    #[test] #[allow(dead_code)]
    pub fn get_neighbor_west()
    {
        let cell = ::GridCell::new(1);
        (*cell.west.borrow_mut()) = Some(2);
        assert![cell.get_neighbor(Neighbor::West).unwrap() == 2];
    }
}


