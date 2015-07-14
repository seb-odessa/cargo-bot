//! The definition for the GridCell
//!
//! GridCell contains it's uniq id and <b>could</b> contain ids of the nearest neighbors
//!
//! ```
//!extern crate lib;
//!use lib::gridcell::{GridCell, Neighbor};
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
//!    cell.add_neighbor(Neighbor::North, north.id);
//!    assert![cell.north.unwrap() == north.id];
//!    assert![cell.get_neighbor(Neighbor::North).unwrap() == north.id]; 
//!}
//! ```


use std::fmt;
pub type Id = usize;

#[derive(Debug, Clone, Copy)]
pub enum Neighbor{ North, South, East, West }

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
            None => "None".to_string() 
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
    pub fn add_neighbor(&mut self, dir: Neighbor, id: Id) -> () { 
        match dir {
            Neighbor::North => self.north = Some(id),
            Neighbor::South => self.south = Some(id),
            Neighbor::East =>  self.east = Some(id),
            Neighbor::West =>  self.west =Some(id),
        }
    }

    #[allow(dead_code)]
    pub fn get_neighbor(&self, dir: Neighbor) -> Option<Id> { 
        match dir {
            Neighbor::North => self.north,
            Neighbor::South => self.south,
            Neighbor::East =>  self.east,
            Neighbor::West =>  self.west,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GridCell, Neighbor};

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
    pub fn add_neighbor_north()
    {
        let mut cell = GridCell::new(1);
        cell.add_neighbor(Neighbor::North, GridCell::new(2).id);
        assert![cell.north.unwrap() == 2];
    } 

    #[test]
    pub fn add_neighbor_south()
    {
        let mut cell = GridCell::new(1);
        cell.add_neighbor(Neighbor::South, GridCell::new(2).id);
        assert![cell.south.unwrap() == 2];
    }

    #[test]
    pub fn add_neighbor_east()
    {
        let mut cell = ::GridCell::new(1);
        cell.add_neighbor(Neighbor::East, GridCell::new(2).id);
        assert![cell.east.unwrap() == 2];
    }

    #[test] 
    pub fn add_neighbor_west()
    {
        let mut cell = ::GridCell::new(1);
        cell.add_neighbor(Neighbor::West, GridCell::new(2).id);
        assert![cell.west.unwrap() == 2];
    }

    #[test]
    pub fn get_neighbor_north()
    {
        let mut cell = GridCell::new(1);
        cell.north = Some(2);
        assert![cell.get_neighbor(Neighbor::North).unwrap() == 2];
    } 

    #[test] 
    pub fn get_neighbor_south()
    {
        let mut cell = GridCell::new(1);
        cell.south = Some(2);
        assert![cell.get_neighbor(Neighbor::South).unwrap() == 2];
    }

    #[test]
    pub fn get_neighbor_east()
    {
        let mut cell = ::GridCell::new(1);
        cell.east = Some(2);
        assert![cell.get_neighbor(Neighbor::East).unwrap() == 2];
    }
    
    #[test]
    pub fn get_neighbor_west()
    {
        let mut cell = ::GridCell::new(1);
        cell.west = Some(2);
        assert![cell.get_neighbor(Neighbor::West).unwrap() == 2];
    }
}


