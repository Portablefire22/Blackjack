#![allow(non_snake_case)]

use colored::Colorize;
use rand::Rng;
mod cards;
fn main() {
    println!("{}", "Blackjack".bold().cyan());
    gameLoop();
}
/*
 *  This shit should have been commented a long time ago.
 *  Guess I should be doing it right now before starting any more.
 *  Also I have like 4 hours until my maths exam so that is cool.
 *  I'll be quickly revising for it at 6 am I imagine. 
 *  Not too bothered by it since I should already have 65% on the module.
 */

fn betting(mut balance: isize) -> (isize, usize) {
    let mut bet = String::new();
    
    
    /*
     *  This section deals with the reading of a bet.
     *  Currently also handles the deducation of balance but that'll likely
     *  be shifted to a different function.
     */
    while String::from(&bet).len() <= 1 { 
        bet = String::from("");
        println!("Balance: {}{}", "£".green().bold(),
            balance.to_string().green().bold()); // Displays the user's current balance.
        println!("Make your bet!:");

        std::io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line"); // Check if the input is even readable.

        let bet: usize = match bet 
            .trim()
            .parse::<usize>(){
                Ok(num) => num,
                Err(_) => continue,
            }; // Removes all whitespace and converts to unsigned integer.

           //.expect("{} {}",String::from(bet).red().bold(), "is not a valid number!".red().bold());
    }
    let bet: usize = match bet 
        .trim()
        .parse::<usize>(){
            Ok(num) => num,
            Err(_) => unreachable!(),
        }; // This for some reason does what should be happening in the while loop?
    
    balance = balance - (bet as isize); // Test deducation, remove this once gameplay is working.          
    
    println!("{:?}", bet); // Output the placed bet. Don't think this does anything but debug.
    let bet: usize = 0; // Resets the bet. Idk why I set this. If the bet is going to be passed.
    return (balance, bet); 
}

fn gameLoop(){ // Main gameloop, this is where everything will be called from.
    let mut balance: isize = 100; // Initial starting balance.                              

    /*
     *  The building of a clean deck for the start of the game is currently unneeded, whilst the
     *  resources used to generate the deck are insignificant it could be reduced by first time
     *  generating a desk and just saving it into a txt file.
     *  The generation function was more so made so that I didn't have to make a deck myself, and
     *  so that it could be further expanded to allow for things such as multiple decks to be used
     *  in a game.
     */
    let cleanDeck = cards::build_deck(); // Builds a clean, unshuffled deck. 
    let shuffledDeck = shuffleDeck(cleanDeck);
    'Gameloop: loop{
        let bettingResults = betting(balance); // Retrieves the results from betting.
        balance = bettingResults.0;
        let mut bet: usize = bettingResults.1;
        
    }
}

fn shuffleDeck(reference: Vec<cards::Card>) -> Vec<cards::Card>{ // Self-explanatory
   /*
    *  For ease of use down the line the generated deck is not directly modified, instead the deck
    *  is cloned (in this case it is cloned into 'unshuffled') and the clone is manipulated
    *  instead. This was done just to prevent having to run the card builder multiple times per
    *  game. 
    *  Insignificant in something like this, but I feel like it's a good habit to learn to lower
    *  processing time on shit hardware, even if it increases the RAM usage.
    */
    let mut shuffled = Vec::new();
    let mut unshuffled = reference.clone();
    loop {
        let mut unshuffledLength = unshuffled.len(); // Use this to make sure the deck contains
                                                     // cards.
        let pos: usize;
        if unshuffledLength != 1 { // If the deck contains more than one card then randomly select
                                   // a vector position within the deck.
            pos = rand::thread_rng().gen_range(0..unshuffledLength);
        } else {
            pos = 0;               // Else select the only card.
        }
        shuffled.push(unshuffled.remove(pos)); // This should remove the card from unshuffled and
                                               // move it into the shuffled vector.
        println!("{:#?}, {:#?}", shuffled.len(),shuffled); // Debug.
        if (unshuffled.len() <= 0) { // Stop the loop once the deck has been shuffled.
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


