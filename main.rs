mod card;

use card::{Card, Deck, verify, monte_carlo_simulation, monte_carlo_with_community, SimulationResults, bulk_monte_carlo_simulation, print_bulk_results, export_to_csv, export_summary_to_csv};
use std::io;
use std::time::Instant;

fn main() {
    println!("=== Texas Hold'em Monte Carlo Analysis ===");
    
    // Ask user for number of simulations per hand
    println!("This will run Monte Carlo simulations for all 1,326 possible starting hands.");
    
    let simulations_per_hand = loop {
        println!("Enter simulations per hand (recommended: 10-100): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => break num,
            _ => println!("Please enter a positive number"),
        }
    };
    
    let total_simulations = 1326 * simulations_per_hand;
    println!("Total simulations to run: {}", total_simulations);
    println!("This may take a while...\n");
    
    let start_time = Instant::now();
    let results = bulk_monte_carlo_simulation(simulations_per_hand);
    let duration = start_time.elapsed();
    
    // Print top 50 results by default
    print_bulk_results(&results, Some(50));
    
    println!("\n=== Performance ===");
    println!("Total time: {:.2}s", duration.as_secs_f64());
    println!("Simulations per second: {:.0}", total_simulations as f64 / duration.as_secs_f64());
    
    // Automatically export to CSV files
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let csv_filename = format!("poker_results_{}.csv", timestamp);
    let summary_filename = format!("poker_summary_{}.csv", timestamp);
    
    if let Err(e) = export_to_csv(&results, &csv_filename) {
        println!("Error exporting to CSV: {}", e);
    }
    
    if let Err(e) = export_summary_to_csv(&results, &summary_filename, simulations_per_hand, duration) {
        println!("Error exporting summary: {}", e);
    }
    
    // Ask if user wants to see more results
    loop {
        println!("\nOptions:");
        println!("1. Show top 100 hands");
        println!("2. Show all hands");
        println!("3. Show bottom 50 hands");
        println!("4. Export custom CSV (choose filename)");
        println!("5. Exit");
        print!("Enter choice (1-5): ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim() {
            "1" => print_bulk_results(&results, Some(100)),
            "2" => print_bulk_results(&results, None),
            "3" => {
                let start_idx = results.len().saturating_sub(50);
                let bottom_results = &results[start_idx..];
                println!("\n=== Bottom 50 Hands ===");
                print_bulk_results(bottom_results, None);
            },
            "4" => {
                println!("Enter filename (without .csv extension): ");
                let mut filename_input = String::new();
                io::stdin().read_line(&mut filename_input).expect("Failed to read line");
                let filename = format!("{}.csv", filename_input.trim());
                
                if let Err(e) = export_to_csv(&results, &filename) {
                    println!("Error exporting to CSV: {}", e);
                }
            },
            "5" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice, please enter 1-5"),
        }
    }
}

fn play_single_hand() {
    println!("\n=== Single Hand Mode ===");
    println!("Enter your two cards using numbers:");
    println!("Suits: 1=Spades, 2=Hearts, 3=Diamonds, 4=Clubs");
    println!("Ranks: 1=Ace, 2-10=Number cards, 11=Jack, 12=Queen, 13=King");

    // Create a deck to track used cards
    let mut deck = Deck::new();

    // Get user's hand
    let card1 = get_card_from_user("Enter first card (rank suit): ", &mut deck);
    let card2 = get_card_from_user("Enter second card (rank suit): ", &mut deck);
    let user_hand = [card1, card2];

    println!("\nYour hand: {} {}", user_hand[0], user_hand[1]);

    // Generate opponent's hand from remaining cards
    let opp_card1 = deck.draw().expect("Failed to draw opponent card 1");
    let opp_card2 = deck.draw().expect("Failed to draw opponent card 2");
    let opponent_hand = [opp_card1, opp_card2];

    println!("Opponent's hand: {} {}", opponent_hand[0], opponent_hand[1]);

    // Generate community cards (flop, turn, river)
    let mut community_cards = Vec::new();
    for _ in 0..5 {
        if let Some(card) = deck.draw() {
            community_cards.push(card);
        }
    }

    println!("\nCommunity cards:");
    println!("Flop: {} {} {}", community_cards[0], community_cards[1], community_cards[2]);
    println!("Turn: {}", community_cards[3]);
    println!("River: {}", community_cards[4]);

    // Evaluate hands and determine winner
    let (winner, user_eval, opp_eval) = verify(&user_hand, &opponent_hand, &community_cards);

    println!("\n=== Hand Evaluation ===");
    println!("Your hand: {}", user_eval.rank);
    println!("Opponent's hand: {}", opp_eval.rank);
    println!("Winner: {}", winner);
}

fn run_preflop_simulation() {
    println!("\n=== Monte Carlo Simulation (Pre-flop) ===");
    println!("Enter your two cards using numbers:");
    println!("Suits: 1=Spades, 2=Hearts, 3=Diamonds, 4=Clubs");
    println!("Ranks: 1=Ace, 2-10=Number cards, 11=Jack, 12=Queen, 13=King");

    let card1 = get_card_input("Enter first card (rank suit): ");
    let card2 = get_card_input("Enter second card (rank suit): ");
    let user_hand = [card1, card2];

    println!("\nYour hand: {} {}", user_hand[0], user_hand[1]);
    
    // Get number of simulations
    let num_sims = get_simulation_count();
    
    println!("\nRunning {} simulations...", num_sims);
    let start_time = Instant::now();
    
    let results = monte_carlo_simulation(&user_hand, num_sims);
    
    let duration = start_time.elapsed();
    
    print_simulation_results(&results, duration);
}

fn run_simulation_with_community() {
    println!("\n=== Monte Carlo Simulation (With Community Cards) ===");
    println!("Enter your two cards using numbers:");
    println!("Suits: 1=Spades, 2=Hearts, 3=Diamonds, 4=Clubs");
    println!("Ranks: 1=Ace, 2-10=Number cards, 11=Jack, 12=Queen, 13=King");

    let card1 = get_card_input("Enter first card (rank suit): ");
    let card2 = get_card_input("Enter second card (rank suit): ");
    let user_hand = [card1, card2];

    println!("\nYour hand: {} {}", user_hand[0], user_hand[1]);
    
    // Get community cards
    println!("\nEnter known community cards (press Enter with no input when done):");
    let mut community_cards = Vec::new();
    
    for i in 1..=5 {
        let prompt = match i {
            1 => "Flop card 1 (or Enter to skip): ",
            2 => "Flop card 2 (or Enter to skip): ",
            3 => "Flop card 3 (or Enter to skip): ",
            4 => "Turn card (or Enter to skip): ",
            5 => "River card (or Enter to skip): ",
            _ => "Card: ",
        };
        
        if let Some(card) = get_optional_card_input(prompt) {
            community_cards.push(card);
        } else {
            break;
        }
    }
    
    if !community_cards.is_empty() {
        println!("\nKnown community cards:");
        for (i, card) in community_cards.iter().enumerate() {
            println!("  {}: {}", i + 1, card);
        }
    }
    
    // Get number of simulations
    let num_sims = get_simulation_count();
    
    println!("\nRunning {} simulations...", num_sims);
    let start_time = Instant::now();
    
    let results = monte_carlo_with_community(&user_hand, &community_cards, num_sims);
    
    let duration = start_time.elapsed();
    
    print_simulation_results(&results, duration);
}

fn get_simulation_count() -> usize {
    loop {
        println!("Enter number of simulations (default 1000): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let input = input.trim();
        if input.is_empty() {
            return 1000;
        }
        
        match input.parse::<usize>() {
            Ok(num) if num > 0 => return num,
            _ => println!("Please enter a positive number"),
        }
    }
}

fn print_simulation_results(results: &SimulationResults, duration: std::time::Duration) {
    println!("\n=== Simulation Results ===");
    println!("Total games: {}", results.total_games);
    println!("Wins: {} ({:.2}%)", results.wins, results.win_rate);
    println!("Losses: {} ({:.2}%)", results.losses, 100.0 - results.win_rate - results.tie_rate);
    println!("Ties: {} ({:.2}%)", results.ties, results.tie_rate);
    println!("Simulation time: {:.2}s", duration.as_secs_f64());
    println!("Games per second: {:.0}", results.total_games as f64 / duration.as_secs_f64());
}

fn get_card_input(prompt: &str) -> Card {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        if parts.len() != 2 {
            println!("Please enter exactly two numbers (rank and suit)");
            continue;
        }

        let rank_num: u8 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid rank number. Please enter 1-13");
                continue;
            }
        };

        let suit_num: u8 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid suit number. Please enter 1-4");
                continue;
            }
        };

        match Card::from_numbers(rank_num, suit_num) {
            Some(card) => return card,
            None => {
                println!("Invalid card values. Rank must be 1-13, suit must be 1-4");
                continue;
            }
        }
    }
}

fn get_optional_card_input(prompt: &str) -> Option<Card> {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.is_empty() {
            return None;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.len() != 2 {
            println!("Please enter exactly two numbers (rank and suit) or press Enter to skip");
            continue;
        }

        let rank_num: u8 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid rank number. Please enter 1-13");
                continue;
            }
        };

        let suit_num: u8 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid suit number. Please enter 1-4");
                continue;
            }
        };

        match Card::from_numbers(rank_num, suit_num) {
            Some(card) => return Some(card),
            None => {
                println!("Invalid card values. Rank must be 1-13, suit must be 1-4");
                continue;
            }
        }
    }
}

fn get_card_from_user(prompt: &str, deck: &mut Deck) -> Card {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        if parts.len() != 2 {
            println!("Please enter exactly two numbers (rank and suit)");
            continue;
        }

        let rank_num: u8 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid rank number. Please enter 1-13");
                continue;
            }
        };

        let suit_num: u8 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid suit number. Please enter 1-4");
                continue;
            }
        };

        match Card::from_numbers(rank_num, suit_num) {
            Some(card) => {
                // Try to add the card to used cards (remove from deck)
                if deck.used_cards().contains(&card) {
                    println!("Card {} has already been used! Please choose another card.", card);
                    continue;
                }
                
                // Remove the card from the deck
                if let Err(e) = deck.remove_card(&card) {
                    println!("Error removing card from deck: {}", e);
                    continue;
                }
                
                return card;
            },
            None => {
                println!("Invalid card values. Rank must be 1-13, suit must be 1-4");
                continue;
            }
        }
    }
}

