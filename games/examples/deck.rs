extern crate games;
use self::games::Deck;
fn main() {
    println!("\nWelcome to Three Card Draw Poker! 🃏");
    println!(
        r"
┌─────────┐  ┌─────────┐  ┌─────────┐
│1        │  │2        │  │3        │
│         │  │         │  │         │
│    1    │  │    2    │  │    3    │
│         │  │         │  │         │
│        1│  │        2│  │        3│
└─────────┘  └─────────┘  └─────────┘

"
    );
    let mut deck = Deck::new();
    println!("🎲 Your starting balance is ${}", deck.balance);

    loop {
        println!("\n🎲 Your current balance is ${}", deck.balance);
        println!("🎲 Enter a bet amount or 0 to quit:");
        let mut bet = String::new();
        std::io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");
        let bet: u32 = match bet.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if bet == 0 {
            println!("👋 Thanks for playing!");
            break;
        } else {
            deck.place_bet(bet);
            deck.determine_winnings();
        }
    }
}
