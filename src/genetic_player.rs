use player::Player;
use c4::C4;
use board::{Board, BoardItem};
use random_player::RandomPlayer;

pub struct GeneticPlayer {
	player_type: BoardItem,
	random_inner: RandomPlayer,
	iteration: usize,
	current_iteration_moves: Vec<(usize, usize, f32)>
}

impl GeneticPlayer {
	pub fn new(ptype: BoardItem) -> GeneticPlayer {
		GeneticPlayer {
			player_type: ptype,
			random_inner: RandomPlayer::new(ptype),
			iteration: 0,
			current_iteration_moves: Vec::new()
		}
	}

	fn fitness(x: usize, y: usize) {
		
	}
}

impl Player for GeneticPlayer {
	fn take_go(&mut self, board: &mut C4) {
	}
}