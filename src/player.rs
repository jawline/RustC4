use c4::C4;

pub trait Player {
	fn take_go(board: &mut C4);
}