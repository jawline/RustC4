use c4::C4;

pub trait Player {
	fn take_go(&mut self, board: &mut C4);
}