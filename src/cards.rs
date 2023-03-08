struct Card {
    suit: String,
    value: u8,
}

fn build_card(cardSuit: String, cardValue: u8) {
    let mut card1 = Card {
        suit = cardSuit,
        value = cardValue,
    } 
    return card1;
}
