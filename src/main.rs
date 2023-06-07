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
    'Gameloop: loop {
        game = betting(game); // Retrieves the results from betting
        for i in 0..2 { // Initial Cards 
            game.player.deck.addCard(shuffledDeck.cards.remove(0));
            game.dealer.deck.addCard(shuffledDeck.cards.remove(0));
        }
        game.checkVictory();
        if (game.checkBankruptcy()) { break 'Gameloop; }
    }
    return game 
}
