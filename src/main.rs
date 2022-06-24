use ansi_term::Colour;
use rand::Rng;
use std::io;
mod data;

fn answer() -> String {
    println!("Hit or Stand?");
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read line.");

    let input_user = input_user.trim_end();

    return input_user.to_string();
}

pub fn result(player: u8, dealer: u8) {
    if player > 21 {
        data::lost(player, dealer);
    } else if dealer > 21 {
        data::win(player, dealer);
    } else if player > dealer {
        data::win(player, dealer);
    } else {
        data::lost(player, dealer);
    }
}
pub fn main() {
    let mut rng = rand::thread_rng();
    let player: u8 = data::value();
    let dealer: u8 = data::value();
    let add = rng.gen_range(1..10);

    data::status(player, dealer);

    let ask = answer();

    if ask == "hit" {
        let player: u8 = player + add;
        let dealer: u8 = dealer + add;

        data::status(player, dealer);

        let ask = answer();

        if ask == "hit" {
            let player: u8 = player + add;
            let dealer: u8 = dealer + add;
            data::status(player, dealer);
            let dealer: u8 = dealer + add;
            println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);

            result(player, dealer);
        } else if ask == "stand" {
            let dealer: u8 = dealer + add;
            data::status(player, dealer);

            result(player, dealer);
        } else {
            result(player, dealer);
        }
    } else if ask == "stand" {
        let dealer: u8 = dealer + add;
        data::status(player, dealer);
        let dealer: u8 = dealer + add;
        println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);

        result(player, dealer);
    } else {
        println!("Please, choose some option");
    }
}
