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

	fn find_row_for_insert(&self, col: usize) -> Option<usize> {
		(0..self.board.height()).find(|&row| !self.board.get(col, row))
	}

	pub fn insertable_columns(&self) -> Vec<usize> {
		(0..self.board.width()).filter(|&col| self.can_insert(col)).collect()
	}

	pub fn can_insert(&self, col: usize) -> bool {
		return self.find_row_for_insert(col).is_some()
	}

	pub fn insert(&mut self, col: usize) -> bool {
		let row = self.find_row_for_insert(col);

		if row.is_some() {
			self.board.set(true, col, row.unwrap());
			true
		} else {
			false
		}
	}

	pub fn is_won() -> bool {
		false
	}

	pub fn print(&self) {
		self.board.print();
	}
}