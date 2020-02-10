use rand::Rng;

#[derive(Debug)]
pub struct Dice{
	value: u8,
}

impl Dice {
	pub fn new() -> Dice {
		let mut d = Dice {
			value: 6,
		};
		d.roll();
		d
	}
	pub fn get_value(&self) -> u8 {
		self.value
	}
	pub fn roll(&mut self){
		self.value = rand::thread_rng().gen_range(1, 7);
	}
}


