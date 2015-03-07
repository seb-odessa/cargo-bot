
use std::sync::{Once, ONCE_INIT};
use std::sync::{StaticMutex, MUTEX_INIT};

static LOCK: StaticMutex = MUTEX_INIT;
static INIT: Once = ONCE_INIT;
static mut id : usize = 0;

pub struct Gen;
impl Gen {
	#[allow(dead_code)] 
	pub fn get() -> usize {		
		unsafe {
			let _guard = LOCK.lock();
			INIT.call_once(|| { id = 0; } );
			id = id + 1;
			return id;
		}
	}
}

