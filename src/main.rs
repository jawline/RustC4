extern crate rand;

mod player;
mod random_player;
mod target_player;
mod board;
mod c4;

use rand::*;
use board::BoardItem;
use player::Player;
use random_player::RandomPlayer;

fn print_round(b1: &c4::C4, round: usize) {
	println!("Round: {}", round);
	b1.print();
}

fn main() {
    let mut b1 = c4::C4::new();
    let mut rng = rand::thread_rng();
    let mut players = [RandomPlayer::new(BoardItem::Naught), RandomPlayer::new(BoardItem::Cross)];

    let mut round = 0;

    b1.print();
    print_round(&b1, round);

    while b1.insertable_columns().len() > 0 && !b1.is_won() {
    	players[round % 2].take_go(&mut b1);
    	round = round + 1;
    	print_round(&b1, round);
    }
}
