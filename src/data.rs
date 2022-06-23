use ansi_term::Colour;
use rand::Rng;
use std::{thread, time};

pub fn value() -> u8 {
    let mut rng = rand::thread_rng();
    let value: u8 = rng.gen_range(1..10);

    return value;
}

pub fn status(player: u8, dealer: u8) {
    let ten_millis = time::Duration::from_millis(1000);

    println!("--------------------------------");
    println!("You {}", Colour::Blue.paint(player.to_string()),);
    thread::sleep(ten_millis);
    println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);
    thread::sleep(ten_millis);
}

pub fn win(player: u8, dealer: u8) {
    println!("--------------------------------");
    println!(
        "{}!!\nYou {}\nDealer {}\n",
        Colour::Green.paint("You win"),
        player,
        dealer
    );
}

pub fn lost(player: u8, dealer: u8) {
    println!("--------------------------------");
    println!(
        "{}!!\nYou {}\nDealer {}\n",
        Colour::Red.paint("You lost"),
        player,
        dealer
    );
}
