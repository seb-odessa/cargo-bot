use std::cell::Cell;
use gridcell::CellId;

#[derive(Debug)]
pub enum Type{ Cargo }

#[allow(dead_code)] 
#[derive(Debug)]
pub struct Bot {
	pub bot : Type,	
	cell : Cell<CellId>,
}

impl Bot {
	#[allow(dead_code)] 
	pub fn cargo() -> Bot {
		Bot {			
			bot : Type::Cargo,
			cell : Cell::new(0),
		} 
	}

	#[allow(dead_code)] 
	pub fn move_to(&self, id : CellId) -> () {
		self.cell.set(id)
	}
}

#[cfg(test)]
mod tests {
    use bot::Bot;
    use bot::Type;
    
    #[test]
    pub fn cargo()
    {
        let bot = Bot::cargo();
        match bot.bot {
        	Type::Cargo => assert!(true),
        }
        assert_eq!(bot.cell.get(), 0);
    }

	#[test]
    pub fn move_to()
    {
        let bot = Bot::cargo();
        bot.move_to(1);
        assert_eq!(bot.cell.get(), 1);
    }
}