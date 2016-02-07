use player::Player;
use c4::C4;
use board::{Board, BoardItem};
use rand::{ThreadRng, Rng, thread_rng};

pub struct RandomPlayer {
	player_type: BoardItem,
	rng: ThreadRng
}

impl RandomPlayer {
	pub fn new(ptype: BoardItem) -> RandomPlayer {
		RandomPlayer{
			player_type: ptype,
			rng: thread_rng()
		}
	}
}

pub fn random_move(board: &mut C4, player_type: BoardItem, rng: &mut ThreadRng) {
	let insertable = board.insertable_columns();

	let pick = if insertable.len() == 1 {
		0
	} else {
		rng.gen::<usize>() % insertable.len()
	};

	board.insert(insertable[pick], player_type);
	println!("Picked {} of {}", pick, insertable.len());	
}

impl Player for RandomPlayer {
	fn take_go(&mut self, board: &mut C4) {
		random_move(board, self.player_type, &mut self.rng);
	}
}