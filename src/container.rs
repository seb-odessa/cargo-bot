#![crate_name = "container"]
#![crate_type = "lib"]
#![feature(box_syntax)]

type List<T> = Option<Box<Node<T>>>;

#[allow(dead_code)] 
#[derive(Debug)]
pub struct Node<T> {
	value: T,
	link: List<T>
}	

#[allow(dead_code)]
impl<T> Node<T> {		
	pub fn new(value: T) -> Node<T> {
    	Node { value: value, link: None, }
	}
	
	pub fn append(&mut self, value: T) {
    	match self.link {
        	Some(ref mut node) => node.append(value),
        	None => self.link = Some(box Node::new(value)),
    	};
	}

	pub fn length(&self) -> usize {
    	match self.link {
        	Some(ref node) => node.length() + 1,
        	None => 1,
    	}
	}
}

