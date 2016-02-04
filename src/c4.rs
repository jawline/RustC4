use board::Board;

pub struct C4 {
	board: Board
}

impl C4 {
	pub fn new() -> C4 {
		C4 {
			board: Board::new()
		}
	}

	pub fn insert(&mut self, col: usize) -> bool {
		let row = (0..self.board.height()).find(|&row| !self.board.get(col, row));

		if row.is_some() {
			self.board.set(true, col, row.unwrap());
			true
		} else {
			false
		}
	}

	pub fn print(&self) {
		self.board.print();
	}
}