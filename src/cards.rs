#[derive(Debug, Clone)]
pub struct Card {
    suit: String,
    value: String,
}

fn build_card(cardSuit: String, cardValue: String) -> Card {
    let mut card1 = Card {
        suit: cardSuit,
        value: cardValue,
    }; 
    return card1;
}

pub fn build_deck() -> Vec<Card>{
    let mut deck = Vec::new();
    let suites: [&str; 4] = ["Hearts","Diamonds","Spades","Clubs"];
    let specials: [&str; 3] = ["King","Queen","Jack"];
    let mut card: Card;
    for suite in suites {
        for i in 1..11 {
            deck.push(build_card(suite.to_string(), i.to_string()));
            if (i == 10) {
                for special in specials {
                    deck.push(build_card(suite.to_string(), special.to_string()));
                }
            }

        }
    }
    return deck;
}

