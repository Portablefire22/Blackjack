


/* 
*
*   This module is going to deal with all of the dealer operations.
*   Operations TODO
*       - Draw cards
*       - Decide if to stand or hit 
*
*/

/*
*
*   Gonna change this so that it's a generic controller.
*   What's the point of doing this shit twice?
*
*/

pub struct CharacterState {
    pub deck: crate::deckHandler::Deck,
    pub controlled: bool,
}


impl CharacterState {
    pub fn drawCards(&mut self, mut freshDeck: crate::deckHandler::Deck, quantity: u8) {
        for i in 0..quantity {
            self.deck.addCard(freshDeck.cards.remove(0));
        }
    }
    pub fn decideMove(&mut self) {
        
    }
}

impl Default for CharacterState {
    fn default() -> CharacterState {
        CharacterState { 
            deck: crate::deckHandler::Deck::default(),
            controlled: false,
        }
    }
}

