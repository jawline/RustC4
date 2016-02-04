extern crate rand;

mod board;
mod c4;

use rand::*;

fn print_round(b1: &c4::C4, round: usize) {
	println!("Round: {}", round);
	b1.print();
}

fn random_insert(b1: &mut c4::C4, rng: &mut rand::ThreadRng) {
	let insertable = b1.insertable_columns();

	let pick = if insertable.len() == 1 {
		0
	} else {
		rng.gen::<usize>() % insertable.len()
	};

	b1.insert(insertable[pick]);
	println!("Picked {} of {}", pick, insertable.len());
}

fn main() {
    let mut b1 = c4::C4::new();
    let mut rng = rand::thread_rng();

    let mut round = 0;

    b1.print();
    print_round(&b1, round);

    while b1.insertable_columns().len() > 0 {
    	random_insert(&mut b1, &mut rng);
    	round = round + 1;
    	print_round(&b1, round);
    }
}
