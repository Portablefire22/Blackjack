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

        let dualDecks = dealInitialCards(shuffledDeck.clone());
        let mut dealerCards = dualDecks.0;
        let mut playerCards = dualDecks.1;
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
                                               // move it into the shuffled vector
        if (unshuffled.len() <= 0) { // Stop the loop once the deck has been shuffled.
            break
        }
    }
    return shuffled
}

fn dealInitialCards(mut deck: Vec<cards::Card>) -> (Vec<cards::Card>, Vec<cards::Card>) {

    // Realistically this would be done in a multi-stage process but I honestly don't care,
    // due to how it was shuffled it shouldn't make a difference.

    // First we shall deal the cards to the dealer.
    let mut dealerCards = Vec::new(); // Vector chosen as more cards may be dealt later. 
    dealerCards.push(deck.remove(0));
    dealerCards.push(deck.remove(0)); // Remove the top card and place it into the dealers hand.

    // Now we shall deal the cards to the player. 
    let mut playerCards = Vec::new();
    playerCards.push(deck.remove(0));
    playerCards.push(deck.remove(0)); // Same as dealer
    drop(deck);
    return (dealerCards, playerCards);
}

fn checkHand (dealerCards: Vec<cards::Card>, playerCards: Vec<cards::Card>) -> (bool, bool, String) {
    let mut isVictory = false;
    let mut isPlayer = false;
    let mut victoryType = "Null";

    // First we check for a natural 21 for the dealer by checking their first hard.
    if ((dealerCards[0].value == 1.to_string() && dealerCards[1].value == 10.to_string()) || ( dealerCards[0].value == 10.to_string() && dealerCards[1].value == 1.to_string())){
        isVictory = true;
        isPlayer = false;
        victoryType = "Natural";
        // If the player also had a natural 21 then it's a tie, this is determined with 
        // isVictory = false
        // isPlayer = true;
        // For quicker and easier checking.
        if ((playerCards[0].value == 1.to_string() && playerCards[1].value == 10.to_string()) || (playerCards[0].value == 10.to_string() && playerCards[1].value == 1.to_string())){
            isVictory = false;
            isPlayer = true;
            victoryType = "Natural Tie";
        }
    }
    // Best to check if it's a victory now,
    if (!(isVictory || isPlayer)){
        // Lets add the cards together.

        // First the dealer 
        let mut dealerTotal: u8 = 0;
        let mut dealerAces: u8 = 0;
        for card in dealerCards {
            if (card.value != 1) {
                dealerTotal = dealerTotal + card.value;
            } else {
                dealerAces = dealerAces + 1; // How do I best integrate this into the calculation.
            }
        }
        
    }


    return (isVictory, isPlayer, victoryType.to_string());
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


