use player::Player;
use c4::C4;
use random_player::random_move;
use board::{Board, BoardItem};
use rand::{ThreadRng, Rng, thread_rng};

pub struct TargetPlayer {
	player_type: BoardItem,
	rng: ThreadRng
}

impl TargetPlayer {
	pub fn new(ptype: BoardItem) -> TargetPlayer {
		TargetPlayer{
			player_type: ptype,
			rng: thread_rng()
		}
	}
}

impl Player for TargetPlayer {
	fn take_go(&mut self, board: &mut C4) {
		random_move(board, self.player_type, &mut self.rng);
	}
}