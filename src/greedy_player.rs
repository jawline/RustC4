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
		let mut best_move = (None, None);
		let mut most_score = 0;

		for w in 0..board.board.width() {
			let h = board.find_row_for_insert(w);
			if h.is_some() {
				//Can take move
				board.board.set(self.player_type, w, h.unwrap());
				let c_score = board.most_score(self.player_type);
				board.board.set(BoardItem::Empty, w, h.unwrap());

				if c_score > most_score {
					most_score = c_score;
					best_move = (Some(w), h);
				}
			}
		}

		let (best_col, _) = best_move;
		board.insert(best_col.unwrap(), self.player_type);
	}
}