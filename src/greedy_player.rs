use player::Player;
use c4::C4;
use board::{Board, BoardItem};

pub struct GreedyPlayer {
	player_type: BoardItem
}

impl GreedyPlayer {
	pub fn new(ptype: BoardItem) -> GreedyPlayer {
		GreedyPlayer {
			player_type: ptype
		}
	}

}

impl Player for GreedyPlayer {
	fn take_go(&mut self, board: &mut C4) {
		for h in 0..board.board.height() {
			for w in 0..board.board.width() {
				if board.board.get(w, h) == BoardItem::Empty {
					//Can take move
				}
			}
		}
	}
}