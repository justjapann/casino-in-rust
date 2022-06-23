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

pub fn main() {
    let mut rng = rand::thread_rng();
    let player: u8 = data::value();
    let dealer: u8 = data::value();

    data::status(player, dealer);

    if answer() == "hit" {
        let player: u8 = player + rng.gen_range(1..10);
        let dealer: u8 = dealer + rng.gen_range(1..10);

        data::status(player, dealer);

        if answer() == "hit" {
            let player: u8 = player + rng.gen_range(1..10);
            let dealer: u8 = dealer + rng.gen_range(1..10);
            data::status(player, dealer);
            let dealer: u8 = dealer + rng.gen_range(1..10);
            println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);

            if player > 21 {
                data::win(player, dealer);
            } else if dealer > 21 {
                data::win(player, dealer);
            } else if player > dealer {
                data::win(player, dealer);
            } else {
                data::lost(player, dealer);
            }
        } else if answer() == "stand" {
            let dealer: u8 = dealer + rng.gen_range(1..10);
            data::status(player, dealer);

            if dealer > 21 {
                data::win(player, dealer);
            } else if player > dealer {
                data::win(player, dealer);
            } else {
                data::lost(player, dealer);
            }
        } else if player > dealer {
            data::win(player, dealer);
        } else {
            data::lost(player, dealer);
        }
    } else if answer() == "stand" {
        let dealer: u8 = dealer + rng.gen_range(1..10);
        data::status(player, dealer);
        let dealer: u8 = dealer + rng.gen_range(1..10);
        println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);

        if dealer > 21 {
            data::win(player, dealer);
        } else if player > dealer {
            data::win(player, dealer);
        } else {
            data::lost(player, dealer);
        }
    } else {
        println!("Please, choose some option");
    }
}
