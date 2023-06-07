#![allow(non_snake_case)]

use colored::Colorize;
use rand::Rng;
mod cards;
mod deckHandler;
mod gameHandler;
mod characterController;

fn main() {
    println!("{}", "Blackjack".bold().cyan());
    let mut game: gameHandler::GameState = gameHandler::GameState::default();
    game.setMultiplier(1);
    game.incrementBalance(100);
    game.setMultiplier(2);
    game.dealer = characterController::CharacterState { deck: deckHandler::Deck::createDeck(Vec::new(), "Player".to_string()), controlled: true, };
    game.dealer = characterController::CharacterState { deck: deckHandler::Deck::createDeck(Vec::new(), "Dealer".to_string()), controlled: false, };
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



fn betting(mut game: gameHandler::GameState) -> gameHandler::GameState {
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
    game.setBet(match bet 
        .trim()
        .parse::<usize>(){
            Ok(num) => num,
            Err(_) => unreachable!(),
        }); // This for some reason does what should be happening in the while loop? 
    return game
}

fn gameLoop(mut game: gameHandler::GameState) -> gameHandler::GameState{ // Main gameloop, this is where everything will be called from.

    let mut cleanDeck = deckHandler::Deck::createDeck(cards::build_deck(), "REFERENCE".to_string());
    let mut shuffledDeck = cleanDeck.shuffle();
    let mut natCheck: bool = true;
    'Gameloop: loop{
        game = betting(game); // Retrieves the results from betting
        for i in 0..2 { // Initial Cardscardsn(34)
            game.player.deck.addCard(shuffledDeck.cards.remove(0));
            game.dealer.deck.addCard(shuffledDeck.cards.remove(0));
        }
        game = checkHand(natCheck, game);
        if (game.checkBankruptcy()) { break 'Gameloop; }
        if (natCheck) { natCheck = !natCheck; }
    }
    return game 
}

/*fn dealInitialCards(mut deck: deckHandler::Deck) -> (Vec<cards::Card>, Vec<cards::Card>) {

    drop(deck);
    return (dealerCards, playerCards);
}*/

fn checkHand (natCheck: bool, mut game: gameHandler::GameState) -> gameHandler::GameState {    
    if (natCheck) {
        let dealerNat = naturalCheck(game.dealer.deck.Clone());
        let playerNat = naturalCheck(game.player.deck.Clone());
        
        if (dealerNat & playerNat) {
            game.setVictory("Tie".to_string(),"NULL".to_string());
        }
        else if (dealerNat) {
            game.setVictory("Natural".to_string(), "Dealer".to_string());
        } else if (playerNat) {
            game.setVictory("Natural".to_string(),"Player".to_string());
        }
    }

    // Best to check if it's a victory now,
    if (game.victoryType == "Null".to_string()){
        // First we check to see if the player has went bust 
       if(game.player.deck.calculateValue() > 21) {
            game.setVictory("Bust".to_string(),"Dealer".to_string());
            return game
        } else if (game.dealer.deck.calculateValue() > 21 ) {// Then we check the dealer if the player isn't bust
            game.setVictory("Bust".to_string(),"Player".to_string());
            return game
        }

       if (!game.player.deck.inPlay && game.player.deck.value < game.dealer.deck.value) {
            game.setVictory("Normal".to_string(), "Dealer".to_string());
            return game
        } else if (!game.dealer.deck.inPlay && game.dealer.deck.value < game.player.deck.value) {
            game.setVictory("Normal".to_string(), "Player".to_string());
            return game
        }
    }
    return game
}

fn naturalCheck(mut deck: deckHandler::Deck) -> bool {
    // First we check for a natural 20 for the dealer by checking their first hard.
    if (deck.calculateValue() == 21){
        return true
    }
    return false
}


