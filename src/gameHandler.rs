#![allow(non_snake_case)]

pub struct GameState { 
    pub victoryType: String,
    pub balance: isize,
    pub victor: String,
    pub bet: usize,
    pub multipler: u8,
    pub player: crate::characterController::CharacterState,
    pub dealer: crate::characterController::CharacterState,

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
        }
    }
}
