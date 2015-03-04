#![feature(box_syntax)]
#![feature(alloc)]

use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;


mod backend {
	use std::rc::Rc;
	use std::rc::Weak;
	use std::cell::RefCell;

	#[allow(dead_code)] #[derive(Debug)]
	pub enum Content { Empty, Bot, Cargo, Dock, Checkpoint }

	type GridCellRc = Rc<RefCell<GridCell>>;
	type GridCellWeak = Option<Weak<RefCell<GridCell>>>;
 	//let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
	#[allow(dead_code)] #[derive(Debug)]
	pub struct GridCell 
	{
		pub id			: usize,
		pub link		: GridCellWeak,
	}

	impl GridCell 
	{
		#[allow(dead_code)]
	    pub fn new(id:usize) -> GridCellRc {
	    	Rc::new( RefCell::new ( GridCell { id : id,	link : None } ) ) 
	    }

		#[allow(dead_code)]
	    pub fn print(&self) -> () {
    		println!("DEMO id : {}, link : {:?}", self.id, self.link);
    		match self.link {
    			Some(ref weak) => println!("DEMO id : {}, Some : {:?}", self.id, weak.upgrade().unwrap()),
    			None => println!("DEMO id : {}, Some : None", self.id),
    		};
	    }

	}
}

extern crate container;
fn main() {
    println!("Hello, world!");

    let mut link = container::Node::new(1);
    link.append(2);
    link.append(3);
    println!("The Link: {:?}", link);


    let cell1 = backend::GridCell::new(1);
    println!("DEMO {:?}", cell1);
    let cell2 = backend::GridCell::new(2);
    println!("DEMO {:?}", cell2);
    cell1.borrow_mut().link = Some(cell2.downgrade());
    
   	cell1.borrow().print();
   	cell2.borrow().print();

    //println!("DEMO {:?}", Some().upgrade().unwrap());

 }
