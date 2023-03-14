#![allow(non_snake_case)]

use colored::Colorize;
use rand::Rng;
mod cards;
fn main() {
    println!("{}", "Blackjack".bold().cyan());
    gameLoop();
}

fn betting(mut balance: isize) -> (isize, usize) {
    let mut bet = String::new();
    while String::from(&bet).len() <= 1 {
        bet = String::from("");
        println!("Balance: {}{}", "£".green().bold(), balance.to_string().green().bold());
        println!("Make your bet!:");
        std::io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");

        let bet: usize = match bet 
            .trim()
            .parse::<usize>(){
                Ok(num) => num,
                Err(_) => continue,
            };
           //.expect("{} {}",String::from(bet).red().bold(), "is not a valid number!".red().bold());
    }
    let bet: usize = match bet 
        .trim()
        .parse::<usize>(){
            Ok(num) => num,
            Err(_) => unreachable!(),
        }; 
    balance = balance - (bet as isize);
    println!("{:?}", bet);
    let bet: usize = 0;
    return (balance, bet);
}

fn gameLoop(){
    let mut balance: isize = 100;
    let cleanDeck = cards::build_deck();
    'Gameloop: loop{
        let bettingResults = betting(balance);
        balance = bettingResults.0;
        let mut bet: usize = bettingResults.1;
        println!("{:#?}", shuffleDeck(cleanDeck.clone()));
        println!("{}", cleanDeck.len());
    }
}

fn shuffleDeck(reference: Vec<cards::Card>) -> Vec<cards::Card>{
    let mut shuffled = Vec::new();
    let mut unshuffled = reference.clone();
    loop {
        let mut unshuffledLength = unshuffled.len();
        let pos: usize;
        if unshuffledLength != 1 {
            pos = rand::thread_rng().gen_range(0..unshuffledLength);
        } else {
            pos = 0;
        }
        shuffled.push(unshuffled[pos].clone());
        //println!("{:#?}", unshuffled[pos]);
        unshuffled.remove(pos);
        println!("{:#?}, {:#?}", unshuffled.len(),unshuffled);
        if (unshuffled.len() <= 0) {
            break
        }
    }
    return shuffled
}
/*fn add_signed(a: isize, b: usize) -> isize{
    if b < 0 {
        return a - (b.abs() as isize);
    } else {
        return a + (b.abs() as isize);
    }
}
fn subtract_signed(a: isize, b: usize) -> isize{
    if a > b as isize {
        return a - b as isize;
    } else if a < b as isize {
        return b as isize - a;
    } else {
        return 0;
    }
}*/


