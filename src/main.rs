use ansi_term::Colour;
use rand::Rng;
use std::io;
use std::{thread, time};

fn answer() -> String {
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read line.");
    let input_user = input_user.trim_end();

    return input_user.to_string();
}

fn main() {
    let ten_millis = time::Duration::from_millis(1000);
    let mut rng = rand::thread_rng();

    let player: u8 = rng.gen_range(1..10);

    let dealer: u8 = rng.gen_range(1..10);

    println!("You {}", Colour::Blue.paint(player.to_string()),);
    thread::sleep(ten_millis);
    println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);
    thread::sleep(ten_millis);
    println!("Hit or Stand?");
    let answer_user = answer();

    if answer_user == "hit" {
        let player: u8 = player + rng.gen_range(1..10);
        let dealer: u8 = dealer + rng.gen_range(1..10);

        println!("--------------------------------");
        println!("You {}", Colour::Blue.paint(player.to_string()),);
        thread::sleep(ten_millis);
        println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);
        thread::sleep(ten_millis);
        println!("Hit or Stand?");
        let answer_user = answer();

        if answer_user == "hit" {
            let player: u8 = player + rng.gen_range(1..10);
            let dealer: u8 = dealer + rng.gen_range(1..10);
            println!("--------------------------------");
            println!("You {}", Colour::Blue.paint(player.to_string()),);
            println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);
            thread::sleep(ten_millis);
            let dealer: u8 = dealer + rng.gen_range(1..10);
            println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);
            thread::sleep(ten_millis);

            if player > 21 {
                println!("--------------------------------");
                println!(
                    "{}!!\nYou {}\nDealer {}\n",
                    Colour::Red.paint("You lost"),
                    player,
                    dealer
                );
            } else if dealer > 21 {
                println!("--------------------------------");
                println!(
                    "{}!!\nYou {}\nDealer {}\n",
                    Colour::Green.paint("You win"),
                    player,
                    dealer
                );
            } else if player > dealer {
                println!("--------------------------------");
                println!(
                    "{}!!\nYou {}\nDealer {}\n",
                    Colour::Green.paint("You win"),
                    player,
                    dealer
                );
            } else {
                println!("--------------------------------");
                println!(
                    "{}!!\nYou {}\nDealer {}\n",
                    Colour::Red.paint("You lost"),
                    player,
                    dealer
                );
            }
        } else if answer_user == "stand" {
            let dealer: u8 = dealer + rng.gen_range(1..10);
            println!("--------------------------------");
            println!("You {}", Colour::Blue.paint(player.to_string()),);
            thread::sleep(ten_millis);
            println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);
            thread::sleep(ten_millis);

            if dealer > 21 {
                println!("--------------------------------");
                println!(
                    "{}!!\nYou {}\nDealer {}\n",
                    Colour::Green.paint("You win"),
                    player,
                    dealer
                );
            } else if player > dealer {
                println!("--------------------------------");
                println!(
                    "{}!!\nYou {}\nDealer {}\n",
                    Colour::Green.paint("You win"),
                    player,
                    dealer
                );
            } else {
                println!("--------------------------------");
                println!(
                    "{}!!\nYou {}\nDealer {}\n",
                    Colour::Red.paint("You lost"),
                    player,
                    dealer
                );
            }
        } else if player > dealer {
            println!("--------------------------------");
            println!(
                "{}!!\nYou {}\nDealer {}\n",
                Colour::Green.paint("You win"),
                player,
                dealer
            );
        } else {
            println!("--------------------------------");
            println!(
                "{}!!\nYou {}\nDealer {}\n",
                Colour::Red.paint("You lost"),
                player,
                dealer
            );
        }
    } else if answer_user == "stand" {
        let dealer: u8 = dealer + rng.gen_range(1..10);
        println!("--------------------------------");
        println!("You {}", Colour::Blue.paint(player.to_string()),);
        thread::sleep(ten_millis);
        println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);
        thread::sleep(ten_millis);
        let dealer: u8 = dealer + rng.gen_range(1..10);
        println!("Dealer {}", Colour::Yellow.paint(dealer.to_string()),);
        thread::sleep(ten_millis);

        if dealer > 21 {
            println!("--------------------------------");
            println!(
                "{}!!\nYou {}\nDealer {}\n",
                Colour::Green.paint("You win"),
                player,
                dealer
            );
        } else if player > dealer {
            println!("--------------------------------");
            println!(
                "{}!!\nYou {}\nDealer {}\n",
                Colour::Green.paint("You win"),
                player,
                dealer
            );
        } else {
            println!("--------------------------------");
            println!(
                "{}!!\nYou {}\nDealer {}\n",
                Colour::Red.paint("You lost"),
                player,
                dealer
            );
        }
    } else {
        println!("Please, choose some option");
    }
}
