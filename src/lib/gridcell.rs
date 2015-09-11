//! The definition for the GridCell
//!
//! GridCell contains it's uniq id and <b>could</b> contain ids of the nearest neighbors
//!
//! ```
//!extern crate lib;
//!use lib::gridcell::{GridCell, Way};
//!
//!fn main() {
//!    let mut cell = GridCell::new(1);
//!    assert![cell.id == 1];
//!    assert![cell.north.is_none()];
//!    assert![cell.south.is_none()];
//!    assert![cell.east.is_none()];
//!    assert![cell.west.is_none()];
//!
//!    let north = GridCell::new(2);
//!    cell.add(Way::North, north.id);
//!    assert![cell.north.unwrap() == north.id];
//!    assert![cell.get(Way::North).unwrap() == north.id]; 
//!}
//! ```


use std::fmt;

pub type Id = usize;

#[derive(Debug, Clone, Copy)]
pub enum Way { North, South, East, West }

#[derive(Debug)]
pub struct GridCell {
    pub id          : Id,
    pub north       : Option<Id>,
    pub south       : Option<Id>,
    pub east        : Option<Id>,
    pub west        : Option<Id>,
}

macro_rules! fmt_id{
    (& $expr : expr) => (
        match $expr { 
	    Some(ref cell) => format!("{}", cell),
	    None => "_".to_string()
        }
    )
}

impl fmt::Display for GridCell {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let north = fmt_id![&self.north];
        let south = fmt_id![&self.south];
        let east  = fmt_id![&self.east];
        let west  = fmt_id![&self.west];
        write!(f, "({:4}, north=>{:4}, south=>{:4}, east=>{:4}, west=>{:4})", self.id, north, south, east, west)
    }
}

impl GridCell {
    #[allow(dead_code)]
    pub fn new(id : Id) -> GridCell {
        GridCell { 
            id    : id,
            north : None,
            south : None,
            east  : None,
            west  : None,
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, dir: Way, id: Id) -> () {
        match dir {
            Way::North => self.north = Some(id),
            Way::South => self.south = Some(id),
            Way::East =>  self.east = Some(id),
            Way::West =>  self.west =Some(id),
        } 
    }

    #[allow(dead_code)]
    pub fn get(&self, dir: Way) -> Option<Id> { 
        match dir {
            Way::North => self.north,
            Way::South => self.south,
            Way::East =>  self.east,
            Way::West =>  self.west,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GridCell, Way};

    #[test] 
    pub fn new()
    {
        let cell = GridCell::new(1);
        assert![1 == cell.id];
        assert![cell.north.is_none()];
        assert![cell.south.is_none()];
        assert![cell.east.is_none()];
        assert![cell.west.is_none()];
    }

    #[test]
    pub fn add_north()
    {
        let mut cell = GridCell::new(1);
        cell.add(Way::North, GridCell::new(2).id);
        assert![cell.north.unwrap() == 2];
    } 

    #[test]
    pub fn add_south()
    {
        let mut cell = GridCell::new(1);
        cell.add(Way::South, GridCell::new(2).id);
        assert![cell.south.unwrap() == 2];
    }

    #[test]
    pub fn add_east()
    {
        let mut cell = ::GridCell::new(1);
        cell.add(Way::East, GridCell::new(2).id);
        assert![cell.east.unwrap() == 2];
    }

    #[test] 
    pub fn add_west()
    {
        let mut cell = ::GridCell::new(1);
        cell.add(Way::West, GridCell::new(2).id);
        assert![cell.west.unwrap() == 2];
    }

    #[test]
    pub fn get_north()
    {
        let mut cell = GridCell::new(1);
        cell.north = Some(2);
        assert![cell.get(Way::North).unwrap() == 2];
    } 

    #[test] 
    pub fn get_south()
    {
        let mut cell = GridCell::new(1);
        cell.south = Some(2);
        assert![cell.get(Way::South).unwrap() == 2];
    }

    #[test]
    pub fn get_east()
    {
        let mut cell = ::GridCell::new(1);
        cell.east = Some(2);
        assert![cell.get(Way::East).unwrap() == 2];
    }
    
    #[test]
    pub fn get_west()
    {
        let mut cell = ::GridCell::new(1);
        cell.west = Some(2);
        assert![cell.get(Way::West).unwrap() == 2];
    }
}


