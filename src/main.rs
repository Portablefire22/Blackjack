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
    game.dealer = characterController::CharacterState { deck: deckHandler::Deck::createDeck(Vec::new(), "Player".to_string()), controlled: true, stand:false, };
    game.dealer = characterController::CharacterState { deck: deckHandler::Deck::createDeck(Vec::new(), "Dealer".to_string()), controlled: false, stand:false, };
    game = gameLoop(game);
    println!("\n");
    if (game.victor == "Dealer".to_string() || game.victor == "The House".to_string()) {
        println!("{}","House always wins bby.".red().bold());
    } else {
        println!("{}","A winner is you!".green().bold());
    }
}

fn betting(mut game: gameHandler::GameState) -> gameHandler::GameState {
    let mut bet = String::new();
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
        }); // Patch fix for not being in scope. :)
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

        // Time to actually do the game loop
        'SecondaryLoop: loop {
            game.displayCards();
            if (game.checkVictory()) {
                //println!("{}", game.checkVictory());
                //0 == 0;
                println!("Victor: {} \nType: {}", game.victor, game.victoryType);
                println!("");
                game.displayCards();
                game.displayDealerCards();
                break 'SecondaryLoop;
            }
            if !game.player.stand {
                'inputLoop: loop { 
                    println!("Make your move: ");
                    let mut playerMove = String::new();
                    std::io::stdin()
                        .read_line(&mut playerMove)
                        .expect("Failed to read line");
                    let playerMove: String = playerMove.trim().to_uppercase();
                    match playerMove.as_str() {
                        "HIT" => {
                            game.player.deck.addCard(shuffledDeck.cards.remove(0));
                            break 'inputLoop;
                        }, 
                        "STAND" => { 
                            game.player.setStand(true);
                            break 'inputLoop;
                        },
                        "DOUBLE" => {
                            game.player.deck.addCard(shuffledDeck.cards.remove(0));
                            game.setBet(game.bet * 2);
                            break 'inputLoop;
                        },
                        _ => println!("Use HIT, STAND, OR DOUBLE")
                    }
                }
            }
            if !game.dealer.stand { shuffledDeck = game.dealer.decideMove(shuffledDeck); }
            }
            'endInputLoop: loop {
                println!("Continue?");
                let mut roundRestart = String::new();
                std::io::stdin()
                    .read_line(&mut roundRestart)
                    .expect("Failed to read line");
                let roundRestart: String = roundRestart.trim().to_uppercase();
                println!("{}", roundRestart.as_str().as_bytes()[0]);
                match roundRestart.as_str().as_bytes()[0] {
                    78 => { // N
                        return game;
                    },
                    89 => { // Y
                        break 'endInputLoop;
                    },
                    _ => println!("{}: {} or {}", "Use", "Yes".green(), "No".red()),
                }
        }
        if (game.checkBankruptcy()) { break 'Gameloop; }
    }
    return game 
}

