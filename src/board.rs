struct Board {
	data: Vec<bool>,
	width: usize,
	height: usize
}

impl Board {

	pub fn new() -> Board {
		Board{
			data: Vec::new(),
			width: 7,
			height: 6
		}
	}

	fn index(&self, col: usize, row: usize) -> usize {
		0
	}

	pub fn set(&mut self, data: bool, col: usize, row: usize) {
		let idx = self.index(col, row);
		self.data[idx] = data;
	}

	pub fn get(&self, col: usize, row: usize) -> bool {
		self.data[self.index(col, row)]
	}
}