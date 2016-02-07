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

	fn find_target(&self, instance: &mut C4) -> Option<usize> {
		
		for h in 0..instance.board.height() {
			for w in 0..instance.board.width() {
				if instance.board.get(w, h) == self.player_type {

					if instance.board.get(w + 1, h) == BoardItem::Empty {
						return Some(w + 1);
					}

					if instance.board.get(w, h + 1) == BoardItem::Empty {
						return Some(w);
					}
				}
			}
		}

		None
	}
}

impl Player for TargetPlayer {

	fn take_go(&mut self, board: &mut C4) {
		match self.find_target(board) {
			Some(w) => {
				board.insert(w, self.player_type);
			},
			_ => random_move(board, self.player_type, &mut self.rng)
		};
	}
}