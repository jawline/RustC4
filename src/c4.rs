use board::{BoardItem, Board};

pub struct C4 {
	pub board: Board
}

impl C4 {

	pub fn new() -> C4 {
		C4 {
			board: Board::new()
		}
	}

	fn find_row_for_insert(&self, col: usize) -> Option<usize> {
		(0..self.board.height()).find(|&row| self.board.get(col, row) == BoardItem::Empty)
	}

	pub fn insertable_columns(&self) -> Vec<usize> {
		(0..self.board.width()).filter(|&col| self.can_insert(col)).collect()
	}

	pub fn can_insert(&self, col: usize) -> bool {
		return self.find_row_for_insert(col).is_some()
	}

	pub fn insert(&mut self, col: usize, i_type: BoardItem) -> bool {
		let row = self.find_row_for_insert(col);
		if row.is_some() {
			self.board.set(i_type, col, row.unwrap());
			true
		} else {
			false
		}
	}

	fn along_four(&self, w: usize, h: usize) -> bool {
		let awh = self.board.get(w, h);
		self.board.get(w + 1, h) == awh && self.board.get(w + 2, h) == awh && self.board.get(w + 3, h) == awh
	}

	fn down_four(&self, w: usize, h: usize) -> bool {
		let awh = self.board.get(w, h);
		self.board.get(w, h + 1) == awh && self.board.get(w, h + 2) == awh && self.board.get(w, h + 3) == awh
	}

	fn diag_four(&self, w: usize, h: usize) -> bool {
		let awh = self.board.get(w, h);
		self.board.get(w + 1, h + 1) == awh && self.board.get(w + 2, h + 2) == awh && self.board.get(w + 3, h + 3) == awh
	}

	pub fn is_won(&self) -> bool {
		for h in 0..self.board.height() {
			for w in 0..self.board.width() {
				if self.board.get(w, h) != BoardItem::Empty {
					if w < self.board.width() - 4 && self.along_four(w, h) {
						return true;
					}

					if h < self.board.height() - 4 && self.down_four(w, h) {
						return true;
					}

					if w < self.board.width() - 4 && h < self.board.height() - 4 && self.diag_four(w, h) {
						return true;
					}
				}
			}
		}
		false
	}

	pub fn print(&self) {
		self.board.print();
	}
}
