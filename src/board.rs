use std::iter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoardItem {
	Empty,
	Naught,
	Cross
}

pub struct Board {
	data: Vec<BoardItem>,
	width: usize,
	height: usize
}

impl Board {

	pub fn new() -> Board {
		Board{
			data: [BoardItem::Empty; 7*6].to_vec(),
			width: 7,
			height: 6
		}
	}

	fn index(&self, col: usize, row: usize) -> usize {
		(self.width * row) + col
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn reset(&mut self) {
		self.data = iter::repeat(BoardItem::Empty).take(self.width * self.height).collect();
	}

	pub fn set(&mut self, data: BoardItem, col: usize, row: usize) {
		let idx = self.index(col, row);
		self.data[idx] = data;
	}

	pub fn get(&self, col: usize, row: usize) -> BoardItem {
		self.data[self.index(col, row)]
	}

	pub fn print(&self) {

		for h in (0..self.height).rev() {
			let mut line = "|".to_string();

			for w in 0..self.width {
				line = line + match self.data[self.index(w, h)] {
					BoardItem::Empty => "*",
					BoardItem::Naught => "O",
					BoardItem::Cross => "X"
				};
			}

			line = line + "|";
			println!("{}", line);
		}
	}
}