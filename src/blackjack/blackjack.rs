#[path = "../data.rs"]
mod data;
use ansi_term::Colour;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct Player {
    name: &'static str,
    _cash: u32,
}

fn answer() -> String {
    println!("Hit or Stand?");
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read line.");

    let input_user = input_user.trim_end();

    return input_user.to_string();
}

pub fn result(player: u8, dealer: u8, cash: u32) -> std::io::Result<()> {
    let _cash = 100;

    if player > 21 {
        data::lost(player, dealer);
        let _cash = cash - 20;
        println!("you actual money is {}", _cash);
    } else if dealer > 21 {
        data::win(player, dealer);
        let _cash = cash + 20;
        println!("you actual money is {}", _cash);
    } else if player > dealer {
        data::win(player, dealer);
        let _cash = cash + 20;
        println!("you actual money is {}", _cash);
    } else {
        data::lost(player, dealer);
        let _cash = cash - 20;
        println!("you actual money is {}", _cash);
    }

    let name: &str = "japan";

    let user = Player { name, _cash };

    let mut file = File::create("foo.txt")?;
    write!(file, "Name: {}\nCash2: {}", user.name, user._cash).expect("Failed to write");
    Ok(())
}

pub fn main() {
    let mut rng = rand::thread_rng();
    let player: u8 = data::value();
    let dealer: u8 = data::value();

    let cash: u32 = 100;

    data::status(player, dealer);

    let ask = answer();

    if ask == "hit" {
        let player: u8 = player + rng.gen_range(1..10);
        let dealer: u8 = dealer + rng.gen_range(1..10);

        data::status(player, dealer);

        let ask = answer();

        if ask == "hit" {
            let player: u8 = player + rng.gen_range(1..10);
            let dealer: u8 = dealer + rng.gen_range(1..10);
            data::status(player, dealer);
            let dealer: u8 = dealer + rng.gen_range(1..10);
            println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);

            result(player, dealer, cash);
        } else if ask == "stand" {
            let dealer: u8 = dealer + rng.gen_range(1..10);
            data::status(player, dealer);

            result(player, dealer, cash);
        } else {
            result(player, dealer, cash);
        }
    } else if ask == "stand" {
        let dealer: u8 = dealer + rng.gen_range(1..10);
        data::status(player, dealer);
        let dealer: u8 = dealer + rng.gen_range(1..10);
        println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);

        result(player, dealer, cash);
    } else {
        println!("Please, choose some option");
    }
}
