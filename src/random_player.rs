use player::Player;
use c4::C4;
use board::{Board, BoardItem};

pub struct RandomPlayer {
	player_type: BoardItem
}

impl RandomPlayer {
	pub fn new(ptype: BoardItem) -> RandomPlayer {
		RandomPlayer{
			player_type: ptype
		}
	}
}

impl Player for RandomPlayer {
	fn take_go(board: &mut C4) {

	}
}