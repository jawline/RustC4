pub struct Board {
	data: Vec<bool>,
	width: usize,
	height: usize
}

impl Board {

	pub fn new() -> Board {
		Board{
			data: [false; 7*6].to_vec(),
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

	pub fn set(&mut self, data: bool, col: usize, row: usize) {
		let idx = self.index(col, row);
		self.data[idx] = data;
	}

	pub fn get(&self, col: usize, row: usize) -> bool {
		self.data[self.index(col, row)]
	}

	pub fn print(&self) {

		for w in 0..self.width {
			let mut line = "|".to_string();

			for h in 0..self.height {
				line = line + match self.data[self.index(w, h)] {
					true => "*",
					false => "-"
				};
			}

			line = line + "|";
			println!("{}", line);
		}
	}
}