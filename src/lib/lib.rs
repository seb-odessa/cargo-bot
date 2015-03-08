//#![feature(alloc)]
#![feature(box_syntax)]
#![feature(std_misc)]

pub use self::gen::Gen;
pub use self::gridcell::CellId;
pub use self::gridcell::GridCell;
pub use self::gridmap::GridMap;

pub mod gen;
pub mod gridcell;
pub mod gridmap;

