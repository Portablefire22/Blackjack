use colored::Colorize;

fn main() {
    println!("{}", "Blackjack".bold().cyan());
    gameLoop();
}

fn betting(mut balance: isize) -> (isize, usize) {
    let mut bet = String::new();
    while String::from(&bet).len() <= 1 {
        bet = String::from("");
        println!("Balance: {}{}", "£".green().bold(), balance.to_string().green().bold());
        println!("Make your bet!:");
        std::io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");

        let bet: usize = match bet 
            .trim()
            .parse::<usize>(){
                Ok(num) => num,
                Err(_) => continue,
            };
           //.expect("{} {}",String::from(bet).red().bold(), "is not a valid number!".red().bold());
    }
    let bet: usize = match bet 
        .trim()
        .parse::<usize>(){
            Ok(num) => num,
            Err(_) => unreachable!(),
        }; 
    balance = balance - bet;
    println!("{:?}", bet);
    let bet: usize = 0;
    return (balance, bet);
}

fn gameLoop(){
    let mut balance: isize = 100;
    'Gameloop: loop{
        let bettingResults = betting(balance);
        balance = bettingResults.0;
        let mut bet: usize = bettingResults.1;
    }
}
