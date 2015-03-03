use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

pub mod backend
{
	#[allow(dead_code)] #[derive(Debug)]
	pub enum Content {
		Empty,
		Bot,
		Cargo,
		Dock,
		Checkpoint,
	}

	#[allow(dead_code)] #[derive(Debug)]
	pub struct GridCell 
	{
		pub id			: usize,
		pub content 	: Content,
		neighbors		: Vec<GridCell>,
		
	}
	impl GridCell 
	{
        pub fn link(mut lhv : Box<GridCell>, mut rhv : Box<GridCell>) -> () 
        {
//            lhv.neighbors.push(rhv);
//            rhv.neighbors.push(lhv);
        }
    }



	#[allow(dead_code)] #[derive(Debug)]
	pub struct Grig<T> {
	    map: Vec<T>
	}

	impl<T> Grig<T> {
        pub fn new() -> Grig<T> {
            Grig {
                map: vec![]
            }
        }

    	pub fn add(&mut self, value: T) -> () {
        	self.map.push(value);
    	}

	}
}


fn main() {
    println!("Hello, world!");
    //let a = Grid{};
}
