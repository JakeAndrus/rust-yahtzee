//mod super::dice_object;
use crate::dice_object::Dice;

pub struct Board {
	dice: Vec<Dice>,
}

impl Board {
	pub fn new() -> Board {
		let mut dice: Vec<Dice> = Vec::new();
		for _val in 1..6 {
			dice.push(Dice::new());
		};
		Board {
			dice: dice
		}
	}

	pub fn show_dice(&self) {
		println!("");
		for (idx, die) in self.dice.iter().enumerate() {
			println!("Dice {} is {:?}", idx+1, die);
		};
	}

	pub fn reroll_dice(&mut self, indices: Vec<u32>) {
		for idx in indices.iter() {
			match self.dice.get_mut(*idx as usize) {
				Some(d) => {
					d.roll();
				},
				None => println!("No die at {}", idx+1)
			}
		}
	}
}