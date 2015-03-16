use std::cell::Cell;
use std::fmt;

pub type CellId = usize;

#[derive(Debug)]
pub enum Neighbor{ North, South, East, West }

#[derive(Debug)]    
pub struct GridCell {
    pub id          : CellId,
    pub north       : Cell<Option<CellId>>,
    pub south       : Cell<Option<CellId>>,
    pub east        : Cell<Option<CellId>>,
    pub west        : Cell<Option<CellId>>,
}

macro_rules! fmt_id{
    ($expr:expr) => (
        match $expr {
            Some(cell) => format!("{}", cell), 
            None => "None".to_string() 
        }
    )
}

impl fmt::Display for GridCell {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let north = fmt_id![self.north.get()];
        let south = fmt_id![self.south.get()];
        let east  = fmt_id![ self.east.get()];
        let west  = fmt_id![ self.west.get()];
        write!(f, "({:4}, north=>{:4}, south=>{:4}, east=>{:4}, west=>{:4})", self.id, north, south, east, west)
    }
}

impl GridCell {
    #[allow(dead_code)]    
    pub fn new(id : CellId) -> GridCell {
        GridCell { 
            id    : id,
            north : Cell::new(None),
            south : Cell::new(None),
            east  : Cell::new(None),
            west  : Cell::new(None),
        } 
    }

    #[allow(dead_code)]
    pub fn add_neighbor(&self, dir: Neighbor, id: CellId) -> () { 
        match dir {
            Neighbor::North => self.north.set(Some(id)),
            Neighbor::South => self.south.set(Some(id)),
            Neighbor::East =>  self.east.set(Some(id)),
            Neighbor::West =>  self.west.set(Some(id)),
        }
    }

    #[allow(dead_code)]
    pub fn get_neighbor(&self, dir: Neighbor) -> Option<CellId> { 
        match dir {
            Neighbor::North => self.north.get(),
            Neighbor::South => self.south.get(),
            Neighbor::East =>  self.east.get(),
            Neighbor::West =>  self.west.get(),
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
        assert![1 == cell.id];
        assert![cell.north.get().is_none()];
        assert![cell.south.get().is_none()];
        assert![cell.east.get().is_none()];
        assert![cell.west.get().is_none()];
    }

    #[test] #[allow(dead_code)]
    pub fn add_neighbor_north()
    {
        let cell = GridCell::new(1);
        cell.add_neighbor(Neighbor::North, GridCell::new(2).id);
        assert![cell.north.get().unwrap() == 2];
    } 

    #[test] #[allow(dead_code)]
    pub fn add_neighbor_south()
    {
        let cell = GridCell::new(1);
        cell.add_neighbor(Neighbor::South, GridCell::new(2).id);
        assert![cell.south.get().unwrap() == 2];
    }

    #[test] #[allow(dead_code)]
    pub fn add_neighbor_east()
    {
        let cell = ::GridCell::new(1);
        cell.add_neighbor(Neighbor::East, GridCell::new(2).id);
        assert![cell.east.get().unwrap() == 2];
    }

    #[test] #[allow(dead_code)]
    pub fn add_neighbor_west()
    {
        let cell = ::GridCell::new(1);
        cell.add_neighbor(Neighbor::West, GridCell::new(2).id);
        assert![cell.west.get().unwrap() == 2];
    }

    #[test] #[allow(dead_code)]
    pub fn get_neighbor_north()
    {
        let cell = GridCell::new(1);
        cell.north.set(Some(2));
        assert![cell.get_neighbor(Neighbor::North).unwrap() == 2];
    } 

    #[test] #[allow(dead_code)]
    pub fn get_neighbor_south()
    {
        let cell = GridCell::new(1);
        cell.south.set(Some(2));
        assert![cell.get_neighbor(Neighbor::South).unwrap() == 2];
    }

    #[test] #[allow(dead_code)]
    pub fn get_neighbor_east()
    {
        let cell = ::GridCell::new(1);
        cell.east.set(Some(2));
        assert![cell.get_neighbor(Neighbor::East).unwrap() == 2];
    }
    
    #[test] #[allow(dead_code)]
    pub fn get_neighbor_west()
    {
        let cell = ::GridCell::new(1);
        cell.west.set(Some(2));
        assert![cell.get_neighbor(Neighbor::West).unwrap() == 2];
    }
}


