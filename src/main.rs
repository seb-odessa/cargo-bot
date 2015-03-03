

pub mod game
{
	#[allow(dead_code)]
	pub struct Grig {
	    map: Vec<i32>
	}

	impl Grig {
    	pub fn add(&mut self, value: i32){
        	self.map.push(value);
    	}
    	pub fn print(&self){
    		println!("map: {:?}", self.map);
    	}
	}
}


fn main() {
    println!("Hello, world!");
    //let a = Grid{};
}
