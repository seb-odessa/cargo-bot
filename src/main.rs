#![feature(box_syntax)]
#![feature(alloc)]


mod backend {
	use std::rc::Rc;
	use std::rc::Weak;
	use std::cell::RefCell;


	type GridCellWrapper = RefCell<GridCell>;
	type GridCellRc = Rc<GridCellWrapper>;
	type GridCellLink = Option<Weak<GridCellWrapper>>;

	#[allow(dead_code)] #[derive(Debug)]
	pub struct GridCell
	{
		pub id			: usize,
		pub link		: GridCellLink,
	}

	impl GridCell
	{
		#[allow(dead_code)]
	    pub fn new(id:usize) -> GridCellRc {
	    	Rc::new( RefCell::new ( GridCell { id : id,	link : None } ) ) 
	    }

		#[allow(dead_code)]
	    pub fn print(&self) -> () {
    		match self.link {
    			Some(ref weak) => println!("DEMO id : {}, Some : {:?}", self.id, weak.upgrade().unwrap()),
    			None => println!("DEMO id : {}, Some : None", self.id),
    		};
	    }

   		#[allow(dead_code)]
	    pub fn link(lhv : GridCellRc, rhv : GridCellRc) -> () {
	    	{
	    		lhv.borrow_mut().link = Some(rhv.downgrade());
	    	};
	    	{
	    		rhv.borrow_mut().link = Some(lhv.downgrade());
	    	};
	    }

	}
}

fn main() {
    let cell1 = backend::GridCell::new(1);    
    let cell2 = backend::GridCell::new(2);    
    let cell3 = backend::GridCell::new(3);
        
   	cell1.borrow().print();
   	cell2.borrow().print();
   	cell3.borrow().print();

	cell1.borrow_mut().link = Some(cell2.downgrade());
    cell2.borrow_mut().link = Some(cell3.downgrade());
    

   	cell1.borrow().print();
   	cell2.borrow().print();
   	cell3.borrow().print();
 }
