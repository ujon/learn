mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s)
}

fn main() {
    let mut b1 = Basket::new(String::from("Apple"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("Brita")]);
    let s2 = Stack::new(vec![10, 20, 30]);

    add_string(&mut b1 , String::from("Cargo"));
    add_string(&mut s1 , String::from("Dragon"));

    println!("(1) {:#?}", b1);
    println!("(2) {:#?}", s1);
}
