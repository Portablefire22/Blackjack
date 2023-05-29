#![allow(non_snake_case)]

use colored::Colorize;
use rand::Rng;
mod cards;
fn main() {
    println!("{}", "Blackjack".bold().cyan());
    let mut game = gameState {
        victoryType: "Null".to_string(),
        balance: 100,
        victor: "Null".to_string(),
        bet: 0,
    };
    game = gameLoop(game);
    println!("\n");
    if (game.victor == "Dealer".to_string() || game.victor == "The House".to_string()) {
        println!("{}","House always wins bby.".red().bold());
    } else {
        println!("{}","A winner is you!".green().bold());
    }
}
/*
 *  This shit should have been commented a long time ago.
 *  Guess I should be doing it right now before starting any more.
 *  Also I have like 4 hours until my maths exam so that is cool.
 *  I'll be quickly revising for it at 6 am I imagine. 
 *  Not too bothered by it since I should already have 65% on the module.
 */

pub struct gameState { 
    pub victoryType: String,
    pub balance: isize,
    pub victor: String,
    pub bet: usize,

}

fn betting(mut game: gameState) -> gameState {
    let mut bet = String::new();
    
    
    /*
     *  This section deals with the reading of a bet.
     *  Currently also handles the deducation of balance but that'll likely
     *  be shifted to a different function.
     */
    while String::from(&bet).len() <= 1 { 
        bet = String::from("");
        println!("Balance: {}{}", "£".green().bold(),
            game.balance.to_string().green().bold()); // Displays the user's current balance.
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
    game.bet = match bet 
        .trim()
        .parse::<usize>(){
            Ok(num) => num,
            Err(_) => unreachable!(),
        }; // This for some reason does what should be happening in the while loop? 
    game.balance = (game.balance - (game.bet as isize));
    return game; 
}

fn gameLoop(mut game: gameState) -> gameState{ // Main gameloop, this is where everything will be called from.
    let mut game = gameState {
        victoryType: "Null".to_string(),
        balance: 100,
        victor: "Null".to_string(),
        bet: 0,
    };
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
    let mut natCheck: bool = true;
    'Gameloop: loop{
        game = betting(game); // Retrieves the results from betting

        let dualDecks = dealInitialCards(shuffledDeck.clone());
        let mut dealerCards = dualDecks.0;
        let mut playerCards = dualDecks.1;
        game = checkHand(dealerCards, playerCards, natCheck, game);
        let bankruptcyResults = checkBankruptcy(game);
        game = bankruptcyResults.0;
        if (bankruptcyResults.1) { break 'Gameloop; }
        if (natCheck) { natCheck = !natCheck; }
    }
    return game 
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

fn checkHand (dealerCards: Vec<cards::Card>, playerCards: Vec<cards::Card>, natCheck: bool, mut game: gameState) -> gameState {    
    if (natCheck) {
        let dealerNat = naturalCheck(dealerCards.clone());
        let playerNat = naturalCheck(playerCards.clone());
        
        if (dealerNat & playerNat) {
            game.victoryType = "Tie".to_string();
        }
        else if (dealerNat) {
            game.victor = "Dealer".to_string();
            game.victoryType = "Natural".to_string();
        } else if (playerNat) {
            game.victor = "Player".to_string();
            game.victoryType = "Natural".to_string();
        }
    }

    // Best to check if it's a victory now,
    if (game.victoryType == "Null".to_string()){
        // Lets add the cards together.
        let dealerTotal: u8 = addCards(dealerCards.clone());
        let playerTotal: u8 = addCards(playerCards.clone());
    }


    return game;
}

fn checkBankruptcy(mut game: gameState) -> (gameState,bool) {
    if (game.victor.clone() != "Player".to_string() && game.balance.clone() < 0){
        game.victor = "The House".to_string();
        game.victoryType = "Bankruptcy".to_string();
        return (game,true)
    }
    return (game,false)
}

fn naturalCheck(deck: Vec<cards::Card>) -> bool {
    // First we check for a natural 20 for the dealer by checking their first hard.
    if (addCards(deck) == 21){
        return true
    }
    return false
}


// Add the cards to their highest collective value.
fn addCards(deck: Vec<cards::Card>) -> u8 {
    let mut deckTotal: u8 = 0;
    let mut deckAces: u8 = 0;
    for card in deck {
       let cardValueInt: u8 = card.value.parse().unwrap(); 
        if (cardValueInt != 1) {
            deckTotal += cardValueInt;
        } else {
            deckAces += 1;
        }
    }
    println!("Before Adding Aces {}", deckTotal);
    for i in 0..deckAces {
        if (deckTotal <= 10) {
            deckTotal += 11;
        } else {
            deckTotal += 1;
        }
    }
    println!("After Adding Aces {}", deckTotal);
    return deckTotal
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


