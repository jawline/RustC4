use player::Player;
use board::BoardItem;

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