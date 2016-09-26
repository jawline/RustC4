extern crate rand;

mod player;
mod random_player;
mod genetic_player;
mod greedy_player;
mod target_player;
mod board;
mod c4;

use rand::*;
use board::BoardItem;
use player::Player;
use greedy_player::GreedyPlayer;
use random_player::RandomPlayer;
use target_player::TargetPlayer;

fn print_round(board: &c4::C4, round: usize) {
	println!("Round: {}", round);
	board.print();
}

fn do_game(board: &mut c4::C4, players: &mut [&mut Player; 2]) {
    board.reset();
    let mut round = 0;
    print_round(board, round);

    while !board.game_over() {
        players[round % 2].take_go(board);
        round = round + 1;
        print_round(board, round);
    }
}

fn main() {
    let mut b1 = c4::C4::new();
    let mut rng = rand::thread_rng();
    let mut players = [&mut GreedyPlayer::new(BoardItem::Naught), &mut GreedyPlayer::new(BoardItem::Cross)] as [&mut Player; 2];

    do_game(&mut b1, &mut players);
}
