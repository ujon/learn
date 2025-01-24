// short cut
// use rand::thread_rng;
use rand::{distributions::Alphanumeric, random, seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    /// associated function === class
    fn new() -> Self {
        let suits = ["Diamonds", "Hearts"];
        let values = ["One", "Two", "Three", "Four"];
        /// Cannot borrow immutable local variable `cards` as mutable
        // let cards = vec![];
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }
        // let deck = Deck { cards: cards };
        // return deck;
        // return Deck { cards: cards };
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    /// ## Arrays & Vector
    /// - An array in Rust has a fixed size and is defined at compile time.
    /// - A vector in Rust is dynamic, allowing its size to grow or shrink at runtime.
    ///
    /// ## Vector
    ///
    /// `vec![]` and `Vec::new()` are equivalent, as both create an empty vector.
    let deck = Deck { cards: vec![] };
    let deck = Deck { cards: Vec::new() };
    let deck = Deck {
        cards: Vec::from([
            String::from("Diamonds of Ace"),
            String::from("Diamonds of Two"),
        ]),
    };
    // #[derive(Debug)] required
    println!("(1) deck: {:?}", deck);
    println!("(2) deck: {}", deck.cards.len());
    println!("(3) deck: {len}", len = deck.cards.len());

    let suits = ["Diamonds", "Hearts"];
    let values = ["One", "Two", "Three", "Four"];
    /// Cannot borrow immutable local variable `cards` as mutable
    // let cards = vec![];
    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{value} of {suit}");
            cards.push(card);
        }
    }
    let deck = Deck { cards: cards };
    println!("(4) deck: {:?}", deck);
    println!("(5) deck: {:#?}", deck);
    let mut deck = Deck::new();
    println!("(6) deck: {:#?}", deck);

    deck.shuffle();
    println!("(7) deck: {:#?}", deck);

    let cards = deck.deal(2);
    println!("(8) cards: {:#?}", cards);
    println!("(9) deck: {:#?}", deck);

    // Must need to add error handling
    // let cards = deck.deal(200);
}
