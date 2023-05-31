#![allow(non_snake_case)]
use rand::Rng;
// Handles the deck
// Builds, shuffles, and calculates values



pub struct Deck {
    pub cards: Vec<crate::cards::Card>,
    pub holder: String,
    pub value: u8,
}



impl Deck {
   pub fn addCards(&mut self) -> u8 {
        self.value = 0; // This sets reference decks to 0 and prevents accidental doubling.
        if (self.holder != "REFERENCE") { // Reference decks contain every card, leading to
                                          // an overflow, so lets just stop that.
            let mut acesTotal: u8 = 0;
            for card in self.cards.clone() { // Iterate through all cards in deck.
                let cardValueInt: u8 = card.value.parse().unwrap(); // Convert to u8.
                if (cardValueInt != 1) { // Add value to total if not ace.
                    self.value += cardValueInt;
                } else { // Increment aces.
                    acesTotal += 1;
                }
            }
            for i in 0..acesTotal {
                if (self.value <= 10) { // If the ace will not exceed 21, add 11 to the deck total.
                    self.value += 11
                } else { // Else add 1.
                    self.value += 1
                }
            }
        }
        return self.value 
    } 

    pub fn createDeck(mut cardsDeck: Vec<crate::cards::Card>, holderTemp: String) -> Deck {
        
        let mut temp = Deck { // Create the deck with an empty value.
            cards: cardsDeck,
            holder: holderTemp,
            value: 0,
        };
        temp.value = temp.Clone().addCards(); // Calculate the total value of the deck. 
        return temp // Return the new deck.
    }

    pub fn Clone(&self) -> Deck { 
        Deck {
            cards: self.cards.clone(),
            holder: self.holder.clone(),
            value: self.value.clone(),
        }
    }

    pub fn shuffle(&mut self) -> Deck {
        /*
        *  For ease of use down the line the generated deck is not directly modified, instead the deck
        *  is cloned (in this case it is cloned into 'unshuffled') and the clone is manipulated
        *  instead. This was done just to prevent having to run the card builder multiple times per
        *  game. 
        *  Insignificant in something like this, but I feel like it's a good habit to learn to lower
        *  processing time on shit hardware, even if it increases the RAM usage.
        */
        let mut shuffled = Deck::default();
        let mut unshuffled = self.Clone();
        loop {
            let mut unshuffledLength = unshuffled.cards.len(); // Use this to make sure the deck contains
                                                        // cards.
            let pos: usize;
            if unshuffledLength != 1 { // If the deck contains more than one card then randomly select
                                    // a vector position within the deck.
                pos = rand::thread_rng().gen_range(0..unshuffledLength);
            } else {
                pos = 0;               // Else select the only card.
            }
            shuffled.cards.push(unshuffled.cards.remove(pos)); // This should remove the card from unshuffled and
                                                // move it into the shuffled vector
            if (unshuffled.cards.len() <= 0) { // Stop the loop once the deck has been shuffled.
                break
            }
        }
        return shuffled

    }
}

impl Default for Deck { // Default values of empty deck. 
    fn default() -> Deck {
        Deck {
            cards: Vec::new(),
            holder: "NULL".to_string(),
            value: 0,
        }
    }
}
