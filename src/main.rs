extern crate rand;

mod player;
mod random_player;
mod board;
mod c4;

use rand::*;
use board::BoardItem;

fn print_round(b1: &c4::C4, round: usize) {
	println!("Round: {}", round);
	b1.print();
}

fn random_insert(b1: &mut c4::C4, board: BoardItem, rng: &mut rand::ThreadRng) {
	let insertable = b1.insertable_columns();

	let pick = if insertable.len() == 1 {
		0
	} else {
		rng.gen::<usize>() % insertable.len()
	};

	b1.insert(insertable[pick], board);
	println!("Picked {} of {}", pick, insertable.len());
}

fn flip(board: BoardItem) -> BoardItem {
	match board {
		BoardItem::Naught => BoardItem::Cross,
		_ => BoardItem::Naught
	}
}

fn main() {
    let mut b1 = c4::C4::new();
    let mut rng = rand::thread_rng();

    let mut round = 0;
    let mut item = BoardItem::Naught;

    b1.print();
    print_round(&b1, round);

    while b1.insertable_columns().len() > 0 && !b1.is_won() {
    	random_insert(&mut b1, item, &mut rng);
    	round = round + 1;
    	print_round(&b1, round);
    	item = flip(item);
    }
}
