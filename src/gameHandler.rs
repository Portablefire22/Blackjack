#![allow(non_snake_case)]

pub struct GameState { 
    pub victoryType: String,
    pub balance: isize,
    pub victor: String,
    pub bet: usize,
    pub multipler: u8,
    pub player: crate::characterController::CharacterState,
    pub dealer: crate::characterController::CharacterState,
    pub natural: bool,
}

impl GameState {
    pub fn setBet(&mut self, value: usize) {
        self.bet = value;
    }
    pub fn incrementBalance(&mut self, value: usize) -> isize {
        self.bet = value; 
        self.balance += (self.bet * (self.multipler as usize)) as isize;
        return self.balance
    }
    pub fn decrementBalance(&mut self, value: usize) -> isize {
        self.bet = value;
        self.balance -= (self.bet * (self.multipler as usize)) as isize;
        return self.balance
    }
    pub fn setVictory(&mut self, vicType: String, victor: String) {
        self.victoryType = vicType;
        self.victor = victor;
    }
    pub fn setMultiplier(&mut self, gains: u8) {
        self.multipler = gains;
    }

    pub fn checkBankruptcy(&mut self) -> bool {
        if (self.victor != "Player".to_string() && self.balance < 0){
            self.setVictory("Bankruptcy".to_string(),"The House".to_string());
            return true
        }
        return false
    }

    pub fn checkVictory(&mut self) -> bool {

        if (self.natural) {
            let dealerNat: bool = self.player.naturalCheck();
            let playerNat: bool = self.dealer.naturalCheck();
            if (dealerNat || playerNat) {
                if (dealerNat && playerNat) {
                    self.setVictory("Tie".to_string(), "NULL".to_string());
                } else if (dealerNat) {
                    self.setVictory("Natural".to_string(), "Dealer".to_string());
                } else if (playerNat) {
                    self.setVictory("Natural".to_string(), "Player".to_string());
                }
                return true // Victory Achieved
            }
            self.natural = false;
        }

        if (self.player.deck.value > 21) {
            self.setVictory("Bust".to_string(), "Dealer".to_string());
            return true
        } else if (self.dealer.deck.value > 21) {
            self.setVictory("Bust".to_string(), "Player".to_string());
            return true
        } else {
            // If player is no longer playing and player has less value than the dealer 
            if (!self.player.deck.inPlay && self.player.deck.value < self.dealer.deck.value) {
                self.setVictory("Normal".to_string(), "Dealer".to_string());
                return true
            } else if (!self.dealer.deck.inPlay && self.dealer.deck.value < self.player.deck.value) {
                self.setVictory("Normal".to_string(), "Player".to_string());
                return true
            }
        }

        return false // No victory achieved 
    }

    pub fn displayCards(&mut self) {
        println!("Current Hand");
        for card in &self.player.deck.cards {
            println!("{} : {}",card.suit, card.name);
        }
    }
    
    pub fn displayDealerCards(&mut self) {
        println!("Dealer Hand");
        for card in &self.dealer.deck.cards {
            println!("{} : {}",card.suit, card.name);
        }
    }

    pub fn resetGame(&mut self) {

    }
}


impl Default for GameState {
    fn default() -> GameState {
        GameState {
            victoryType: "NULL".to_string(),
            balance: 0,
            victor: "NULL".to_string(),
            bet: 0,
            multipler: 0,
            player: crate::characterController::CharacterState::default(),
            dealer: crate::characterController::CharacterState::default(),
            natural: true,
        }
    }
}
