mod game_board;
mod dice_object;
use game_board::Board;
use std::io;
fn main() {
    let mut board = Board::new();
    loop {
        println!("---------------------------------");
        println!("New Turn");
        board.reroll_dice((0..6).collect());
        for turn in 0..2 {
            board.show_dice();
            println!("Enter numbers separated by spaces for the dice you want to reroll.");
            let mut action = String::new();
            io::stdin().read_line(&mut action).expect("Failed to read line");
            let dice_indices: Vec<u32> = action[..]
                .trim()
                .split_whitespace()
                .map(|s| (s.parse::<u32>().expect("Enter an integer")))
                .map(|n| n-1)
                .collect();
            
            println!("Rerolling: {:?}", dice_indices);
            board.reroll_dice(dice_indices);
        }
        println!("");
        println!("Final dice: ");
        board.show_dice();
        println!("Hit enter for next turn");
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy).expect("Failed to read line");
    }
    
}
