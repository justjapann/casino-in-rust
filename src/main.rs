#[path = "./blackjack/blackjack.rs"]
mod blackjack;
#[path = "./blackjack/cards/cards.rs"]
mod cards;
mod data;

fn main() {
    //blackjack::main();
    cards::oneAs();
    cards::fourAs()
}
