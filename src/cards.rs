#[derive(Debug, Clone)] // Required to allow for the values to be cloned, and then displayed to
                        // the screen with debug tools.

// Create the frame that the cards will be placed into.
pub struct Card {
    pub suit: String,
    pub value: String,
    pub name: String,
}

// Places the values into the card struct.
fn build_card(cardSuit: String, cardValue: String, cardName: String) -> Card {
    let mut card1 = Card {
        suit: cardSuit,
        value: cardValue,
        name: cardName,
    }; 
    return card1;
}

pub fn build_deck() -> Vec<Card>{
    let mut deck = Vec::new();
    let suites: [&str; 4] = ["Hearts","Diamonds","Spades","Clubs"]; // Allocates the suites
    let specials: [&str; 3] = ["King","Queen","Jack"]; // Allocates the specials for the suites
    let mut card: Card;
    for suit in suites { // Iterate through every suit
        for i in 1..11 { // Iterate through every card value from 1 to 10
            deck.push(build_card(suit.to_string(), i.to_string(), "Null".to_string())); // Build a card and add it to
                                                                    // the deck 
            if (i == 10) { // Building the specials
                for special in specials { // Iterate through all specials 
                    deck.push(build_card(suit.to_string(),i.to_string(), special.to_string()));
                }
            }

        }
    }
    return deck; // Return the deck for use in the card game.
}

