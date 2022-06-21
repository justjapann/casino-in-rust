use rand::Rng;
use std::io;
use std::{thread, time::Duration};

fn answer(input_string: &str) {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line.");
    let input_string = input_string.trim_end();
}

fn main() {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line.");
    let input_string = input_string.trim_end();
    let pergunta1 = answer(input_string);

    let mut rng = rand::thread_rng();

    let player: u8 = rng.gen_range(0..10);

    let dealer: u8 = rng.gen_range(0..10);

    println!("you {}\ndealer {}\nHit or Stand?", player, dealer);

    if pergunta1 == "hit" {
        let player: u8 = player + rng.gen_range(0..10);
        let dealer: u8 = dealer + rng.gen_range(0..10);

        println!("You {}\nDealer {}\nHit or Stand?", player, dealer);
    } else {
    }
}
