mod board;
mod c4;

fn main() {
    let mut b1 = c4::C4::new();
    b1.insert(3);
    b1.insert(3);
    b1.insert(1);
    b1.insert(0);
    b1.print();
}
