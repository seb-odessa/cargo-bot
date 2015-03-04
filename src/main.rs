#![feature(box_syntax)]


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
	pub north		: Option<Box<GridCell>>,
	pub south		: Option<Box<GridCell>>,
	
	
}
impl GridCell 
{
	#[allow(dead_code)]
    pub fn link(mut lhv : Box<GridCell>, mut rhv : Box<GridCell>) -> () 
    {
            lhv.north = Some(rhv);
//            rhv.south = Some(lhv);
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

extern crate container;

fn main() {
    println!("Hello, world!");

    let mut link = container::Node::new(1);
    link.append(2);
    link.append(3);

    println!("link.length() = {:?}", link.length());
    println!("{:?}", link);

 }
