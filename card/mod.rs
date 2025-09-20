use std::fmt;
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{}", label)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Suit {
    pub fn from_number(num: u8) -> Option<Self> {
        match num {
            1 => Some(Suit::Spades),
            2 => Some(Suit::Hearts),
            3 => Some(Suit::Diamonds),
            4 => Some(Suit::Clubs),
            _ => None,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=4) {
            1 => Suit::Spades,
            2 => Suit::Hearts,
            3 => Suit::Diamonds,
            _ => Suit::Clubs,
        }
    }
}

impl Rank {
    pub fn from_number(num: u8) -> Option<Self> {
        match num {
            1 => Some(Rank::Ace),
            2 => Some(Rank::Two),
            3 => Some(Rank::Three),
            4 => Some(Rank::Four),
            5 => Some(Rank::Five),
            6 => Some(Rank::Six),
            7 => Some(Rank::Seven),
            8 => Some(Rank::Eight),
            9 => Some(Rank::Nine),
            10 => Some(Rank::Ten),
            11 => Some(Rank::Jack),
            12 => Some(Rank::Queen),
            13 => Some(Rank::King),
            _ => None,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=13) {
            1 => Rank::Ace,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            _ => Rank::King,
        }
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn from_numbers(rank_num: u8, suit_num: u8) -> Option<Self> {
        match (Rank::from_number(rank_num), Suit::from_number(suit_num)) {
            (Some(rank), Some(suit)) => Some(Card::new(rank, suit)),
            _ => None,
        }
    }

    pub fn random() -> Self {
        Card::new(Rank::random(), Suit::random())
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
    used_cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        
        // Create a full deck of 52 cards
        for suit_num in 1..=4 {
            for rank_num in 1..=13 {
                if let Some(card) = Card::from_numbers(rank_num, suit_num) {
                    cards.push(card);
                }
            }
        }
        
        Deck {
            cards,
            used_cards: Vec::new(),
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.cards.len());
        let card = self.cards.remove(index);
        self.used_cards.push(card.clone());
        Some(card)
    }

    pub fn add(&mut self, card: Card) -> Result<(), String> {
        // Check if the card is already used
        if self.used_cards.contains(&card) {
            return Err("Card has already been used".to_string());
        }

        // Check if the card is already in the deck
        if self.cards.contains(&card) {
            return Err("Card is already in the deck".to_string());
        }

        // Remove from used cards if it's there and add back to deck
        if let Some(pos) = self.used_cards.iter().position(|c| *c == card) {
            self.used_cards.remove(pos);
        }
        
        self.cards.push(card);
        Ok(())
    }

    pub fn remaining_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn used_cards(&self) -> &[Card] {
        &self.used_cards
    }

    pub fn remove_card(&mut self, card: &Card) -> Result<(), String> {
        if let Some(pos) = self.cards.iter().position(|c| *c == *card) {
            let removed_card = self.cards.remove(pos);
            self.used_cards.push(removed_card);
            Ok(())
        } else {
            Err("Card not found in deck".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
    RoyalFlush = 10,
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            HandRank::HighCard => "High Card",
            HandRank::Pair => "Pair",
            HandRank::TwoPair => "Two Pair",
            HandRank::ThreeOfAKind => "Three of a Kind",
            HandRank::Straight => "Straight",
            HandRank::Flush => "Flush",
            HandRank::FullHouse => "Full House",
            HandRank::FourOfAKind => "Four of a Kind",
            HandRank::StraightFlush => "Straight Flush",
            HandRank::RoyalFlush => "Royal Flush",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
pub struct HandEvaluation {
    pub rank: HandRank,
    pub high_cards: Vec<Rank>,
}

impl HandEvaluation {
    fn new(rank: HandRank, high_cards: Vec<Rank>) -> Self {
        HandEvaluation { rank, high_cards }
    }
}

pub fn evaluate_hand(hole_cards: &[Card; 2], community_cards: &[Card]) -> HandEvaluation {
    let mut all_cards = hole_cards.to_vec();
    all_cards.extend_from_slice(community_cards);
    
    // Sort cards by rank (highest first)
    all_cards.sort_by(|a, b| b.rank.cmp(&a.rank));
    
    // Check for each hand type in order of strength
    if let Some(eval) = check_royal_flush(&all_cards) {
        return eval;
    }
    if let Some(eval) = check_straight_flush(&all_cards) {
        return eval;
    }
    if let Some(eval) = check_four_of_a_kind(&all_cards) {
        return eval;
    }
    if let Some(eval) = check_full_house(&all_cards) {
        return eval;
    }
    if let Some(eval) = check_flush(&all_cards) {
        return eval;
    }
    if let Some(eval) = check_straight(&all_cards) {
        return eval;
    }
    if let Some(eval) = check_three_of_a_kind(&all_cards) {
        return eval;
    }
    if let Some(eval) = check_two_pair(&all_cards) {
        return eval;
    }
    if let Some(eval) = check_pair(&all_cards) {
        return eval;
    }
    
    // High card
    let high_cards = all_cards.iter().take(5).map(|c| c.rank).collect();
    HandEvaluation::new(HandRank::HighCard, high_cards)
}

fn check_royal_flush(cards: &[Card]) -> Option<HandEvaluation> {
    // Check each suit for A-K-Q-J-10
    for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
        let suit_cards: Vec<&Card> = cards.iter().filter(|c| c.suit == suit).collect();
        if suit_cards.len() >= 5 {
            let royal_ranks = [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten];
            if royal_ranks.iter().all(|&rank| suit_cards.iter().any(|c| c.rank == rank)) {
                return Some(HandEvaluation::new(HandRank::RoyalFlush, vec![Rank::Ace]));
            }
        }
    }
    None
}

fn check_straight_flush(cards: &[Card]) -> Option<HandEvaluation> {
    for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
        let mut suit_cards: Vec<&Card> = cards.iter().filter(|c| c.suit == suit).collect();
        suit_cards.sort_by(|a, b| b.rank.cmp(&a.rank));
        
        if let Some(high_card) = find_straight(&suit_cards.iter().map(|c| c.rank).collect::<Vec<_>>()) {
            return Some(HandEvaluation::new(HandRank::StraightFlush, vec![high_card]));
        }
    }
    None
}

fn check_four_of_a_kind(cards: &[Card]) -> Option<HandEvaluation> {
    let rank_counts = count_ranks(cards);
    
    for (rank, count) in rank_counts.iter() {
        if *count >= 4 {
            let kicker = cards.iter()
                .find(|c| c.rank != *rank)
                .map(|c| c.rank)
                .unwrap_or(Rank::Two);
            return Some(HandEvaluation::new(HandRank::FourOfAKind, vec![*rank, kicker]));
        }
    }
    None
}

fn check_full_house(cards: &[Card]) -> Option<HandEvaluation> {
    let rank_counts = count_ranks(cards);
    let mut trips = None;
    let mut pair = None;
    
    for (rank, count) in rank_counts.iter() {
        if *count >= 3 && trips.is_none() {
            trips = Some(*rank);
        } else if *count >= 2 && pair.is_none() {
            pair = Some(*rank);
        }
    }
    
    if let (Some(trips_rank), Some(pair_rank)) = (trips, pair) {
        return Some(HandEvaluation::new(HandRank::FullHouse, vec![trips_rank, pair_rank]));
    }
    None
}

fn check_flush(cards: &[Card]) -> Option<HandEvaluation> {
    for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
        let mut suit_cards: Vec<&Card> = cards.iter().filter(|c| c.suit == suit).collect();
        if suit_cards.len() >= 5 {
            suit_cards.sort_by(|a, b| b.rank.cmp(&a.rank));
            let high_cards = suit_cards.iter().take(5).map(|c| c.rank).collect();
            return Some(HandEvaluation::new(HandRank::Flush, high_cards));
        }
    }
    None
}

fn check_straight(cards: &[Card]) -> Option<HandEvaluation> {
    let ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    if let Some(high_card) = find_straight(&ranks) {
        return Some(HandEvaluation::new(HandRank::Straight, vec![high_card]));
    }
    None
}

fn check_three_of_a_kind(cards: &[Card]) -> Option<HandEvaluation> {
    let rank_counts = count_ranks(cards);
    
    for (rank, count) in rank_counts.iter() {
        if *count >= 3 {
            let mut kickers: Vec<Rank> = cards.iter()
                .filter(|c| c.rank != *rank)
                .map(|c| c.rank)
                .collect();
            kickers.sort_by(|a, b| b.cmp(a));
            kickers.truncate(2);
            
            let mut result = vec![*rank];
            result.extend(kickers);
            return Some(HandEvaluation::new(HandRank::ThreeOfAKind, result));
        }
    }
    None
}

fn check_two_pair(cards: &[Card]) -> Option<HandEvaluation> {
    let rank_counts = count_ranks(cards);
    let mut pairs = Vec::new();
    
    for (rank, count) in rank_counts.iter() {
        if *count >= 2 {
            pairs.push(*rank);
        }
    }
    
    if pairs.len() >= 2 {
        pairs.sort_by(|a, b| b.cmp(a));
        let kicker = cards.iter()
            .find(|c| c.rank != pairs[0] && c.rank != pairs[1])
            .map(|c| c.rank)
            .unwrap_or(Rank::Two);
        
        return Some(HandEvaluation::new(HandRank::TwoPair, vec![pairs[0], pairs[1], kicker]));
    }
    None
}

fn check_pair(cards: &[Card]) -> Option<HandEvaluation> {
    let rank_counts = count_ranks(cards);
    
    for (rank, count) in rank_counts.iter() {
        if *count >= 2 {
            let mut kickers: Vec<Rank> = cards.iter()
                .filter(|c| c.rank != *rank)
                .map(|c| c.rank)
                .collect();
            kickers.sort_by(|a, b| b.cmp(a));
            kickers.truncate(3);
            
            let mut result = vec![*rank];
            result.extend(kickers);
            return Some(HandEvaluation::new(HandRank::Pair, result));
        }
    }
    None
}

fn count_ranks(cards: &[Card]) -> std::collections::HashMap<Rank, usize> {
    let mut counts = std::collections::HashMap::new();
    for card in cards {
        *counts.entry(card.rank).or_insert(0) += 1;
    }
    counts
}

fn find_straight(ranks: &[Rank]) -> Option<Rank> {
    use std::collections::HashSet;
    let unique_ranks: HashSet<Rank> = ranks.iter().cloned().collect();
    let mut sorted_ranks: Vec<Rank> = unique_ranks.into_iter().collect();
    sorted_ranks.sort_by(|a, b| b.cmp(a));
    
    // Check for regular straights
    if sorted_ranks.len() >= 5 {
        for i in 0..=sorted_ranks.len() - 5 {
            let mut consecutive = true;
            for j in 0..4 {
                if sorted_ranks[i + j].value() != sorted_ranks[i + j + 1].value() + 1 {
                    consecutive = false;
                    break;
                }
            }
            if consecutive {
                return Some(sorted_ranks[i]);
            }
        }
    }
    
    // Check for A-2-3-4-5 straight (wheel)
    if sorted_ranks.contains(&Rank::Ace) && 
       sorted_ranks.contains(&Rank::Two) && 
       sorted_ranks.contains(&Rank::Three) && 
       sorted_ranks.contains(&Rank::Four) && 
       sorted_ranks.contains(&Rank::Five) {
        return Some(Rank::Five); // In wheel straight, 5 is the high card
    }
    
    None
}

pub fn verify(hand_a: &[Card; 2], hand_b: &[Card; 2], community_cards: &[Card]) -> (String, HandEvaluation, HandEvaluation) {
    let eval_a = evaluate_hand(hand_a, community_cards);
    let eval_b = evaluate_hand(hand_b, community_cards);
    
    let winner = match eval_a.rank.cmp(&eval_b.rank) {
        std::cmp::Ordering::Greater => "Hand A",
        std::cmp::Ordering::Less => "Hand B",
        std::cmp::Ordering::Equal => {
            // Same hand rank, compare high cards
            match eval_a.high_cards.cmp(&eval_b.high_cards) {
                std::cmp::Ordering::Greater => "Hand A",
                std::cmp::Ordering::Less => "Hand B",
                std::cmp::Ordering::Equal => "Tie",
            }
        }
    };
    
    (winner.to_string(), eval_a, eval_b)
}

#[derive(Debug, Clone)]
pub struct SimulationResults {
    pub total_games: usize,
    pub wins: usize,
    pub losses: usize,
    pub ties: usize,
    pub win_rate: f64,
    pub tie_rate: f64,
}

impl SimulationResults {
    fn new(total_games: usize, wins: usize, losses: usize, ties: usize) -> Self {
        let win_rate = (wins as f64 / total_games as f64) * 100.0;
        let tie_rate = (ties as f64 / total_games as f64) * 100.0;
        
        SimulationResults {
            total_games,
            wins,
            losses,
            ties,
            win_rate,
            tie_rate,
        }
    }
}

pub fn monte_carlo_simulation(player_hand: &[Card; 2], num_simulations: usize) -> SimulationResults {
    let mut wins = 0;
    let mut losses = 0;
    let mut ties = 0;
    
    for _ in 0..num_simulations {
        // Create a new deck for each simulation
        let mut deck = Deck::new();
        
        // Remove player's cards from deck
        for card in player_hand {
            if let Err(_) = deck.remove_card(card) {
                // Card not found in deck, skip this simulation
                continue;
            }
        }
        
        // Deal opponent hand
        let opp_card1 = match deck.draw() {
            Some(card) => card,
            None => continue, // Not enough cards, skip
        };
        let opp_card2 = match deck.draw() {
            Some(card) => card,
            None => continue, // Not enough cards, skip
        };
        let opponent_hand = [opp_card1, opp_card2];
        
        // Deal community cards
        let mut community_cards = Vec::new();
        for _ in 0..5 {
            if let Some(card) = deck.draw() {
                community_cards.push(card);
            } else {
                break; // Not enough cards
            }
        }
        
        // Skip if we don't have enough community cards
        if community_cards.len() < 5 {
            continue;
        }
        
        // Evaluate hands and determine winner
        let (winner, _player_eval, _opp_eval) = verify(player_hand, &opponent_hand, &community_cards);
        
        match winner.as_str() {
            "Hand A" => wins += 1,
            "Hand B" => losses += 1,
            "Tie" => ties += 1,
            _ => {} // Should not happen
        }
    }
    
    SimulationResults::new(num_simulations, wins, losses, ties)
}

pub fn monte_carlo_with_community(
    player_hand: &[Card; 2], 
    known_community: &[Card], 
    num_simulations: usize
) -> SimulationResults {
    let mut wins = 0;
    let mut losses = 0;
    let mut ties = 0;
    
    for _ in 0..num_simulations {
        // Create a new deck for each simulation
        let mut deck = Deck::new();
        
        // Remove player's cards from deck
        for card in player_hand {
            if let Err(_) = deck.remove_card(card) {
                continue;
            }
        }
        
        // Remove known community cards from deck
        for card in known_community {
            if let Err(_) = deck.remove_card(card) {
                continue;
            }
        }
        
        // Deal opponent hand
        let opp_card1 = match deck.draw() {
            Some(card) => card,
            None => continue,
        };
        let opp_card2 = match deck.draw() {
            Some(card) => card,
            None => continue,
        };
        let opponent_hand = [opp_card1, opp_card2];
        
        // Complete community cards
        let mut community_cards = known_community.to_vec();
        let cards_needed = 5 - known_community.len();
        
        for _ in 0..cards_needed {
            if let Some(card) = deck.draw() {
                community_cards.push(card);
            } else {
                break;
            }
        }
        
        // Skip if we don't have enough community cards
        if community_cards.len() < 5 {
            continue;
        }
        
        // Evaluate hands and determine winner
        let (winner, _player_eval, _opp_eval) = verify(player_hand, &opponent_hand, &community_cards);
        
        match winner.as_str() {
            "Hand A" => wins += 1,
            "Hand B" => losses += 1,
            "Tie" => ties += 1,
            _ => {}
        }
    }
    
    SimulationResults::new(num_simulations, wins, losses, ties)
}

#[derive(Debug, Clone)]
pub struct HandResult {
    pub hand: [Card; 2],
    pub hand_description: String,
    pub results: SimulationResults,
}

impl HandResult {
    fn new(hand: [Card; 2], results: SimulationResults) -> Self {
        let hand_description = describe_hand(&hand);
        HandResult {
            hand,
            hand_description,
            results,
        }
    }
}

fn describe_hand(hand: &[Card; 2]) -> String {
    let card1 = &hand[0];
    let card2 = &hand[1];
    
    if card1.rank == card2.rank {
        // Pocket pair
        format!("{}{}(pair)", card1.rank, card1.rank)
    } else if card1.suit == card2.suit {
        // Suited
        let (high, low) = if card1.rank > card2.rank {
            (&card1.rank, &card2.rank)
        } else {
            (&card2.rank, &card1.rank)
        };
        format!("{}{}s", high, low)
    } else {
        // Offsuit
        let (high, low) = if card1.rank > card2.rank {
            (&card1.rank, &card2.rank)
        } else {
            (&card2.rank, &card1.rank)
        };
        format!("{}{}o", high, low)
    }
}

pub fn generate_all_starting_hands() -> Vec<[Card; 2]> {
    let mut hands = Vec::new();
    
    // Get all cards from a fresh deck
    let all_cards: Vec<Card> = (1..=4)
        .flat_map(|suit| {
            (1..=13).filter_map(move |rank| Card::from_numbers(rank, suit))
        })
        .collect();
    
    // Generate all possible 2-card combinations
    for i in 0..all_cards.len() {
        for j in (i + 1)..all_cards.len() {
            hands.push([all_cards[i].clone(), all_cards[j].clone()]);
        }
    }
    
    hands
}

pub fn bulk_monte_carlo_simulation(simulations_per_hand: usize) -> Vec<HandResult> {
    let all_hands = generate_all_starting_hands();
    let mut results = Vec::new();
    
    println!("Running Monte Carlo simulation for {} unique starting hands...", all_hands.len());
    println!("Simulations per hand: {}", simulations_per_hand);
    println!("Total simulations: {}", all_hands.len() * simulations_per_hand);
    println!();
    
    let total_hands = all_hands.len();
    
    for (index, hand) in all_hands.iter().enumerate() {
        if index % 100 == 0 {
            println!("Progress: {}/{} hands completed ({:.1}%)", 
                     index, total_hands, (index as f64 / total_hands as f64) * 100.0);
        }
        
        let simulation_results = monte_carlo_simulation(hand, simulations_per_hand);
        let hand_result = HandResult::new(hand.clone(), simulation_results);
        results.push(hand_result);
    }
    
    println!("Completed all {} hands!", total_hands);
    
    // Sort by win rate (highest first)
    results.sort_by(|a, b| b.results.win_rate.partial_cmp(&a.results.win_rate).unwrap());
    
    results
}

pub fn print_bulk_results(results: &[HandResult], top_n: Option<usize>) {
    let display_count = top_n.unwrap_or(results.len());
    let display_count = display_count.min(results.len());
    
    println!("\n=== Monte Carlo Results (Top {} Hands) ===", display_count);
    println!("{:<12} {:<8} {:<8} {:<8} {:<8} {:<8}", 
             "Hand", "Win%", "Lose%", "Tie%", "Wins", "Total");
    println!("{}", "-".repeat(60));
    
    for (rank, result) in results.iter().take(display_count).enumerate() {
        let lose_rate = 100.0 - result.results.win_rate - result.results.tie_rate;
        println!("{:<3} {:<8} {:<8.2} {:<8.2} {:<8.2} {:<8} {:<8}",
                 rank + 1,
                 result.hand_description,
                 result.results.win_rate,
                 lose_rate,
                 result.results.tie_rate,
                 result.results.wins,
                 result.results.total_games);
    }
    
    if display_count < results.len() {
        println!("\n... and {} more hands", results.len() - display_count);
    }
    
    // Print some statistics
    let avg_win_rate = results.iter().map(|r| r.results.win_rate).sum::<f64>() / results.len() as f64;
    let best_hand = &results[0];
    let worst_hand = &results[results.len() - 1];
    
    println!("\n=== Summary Statistics ===");
    println!("Average win rate: {:.2}%", avg_win_rate);
    println!("Best hand: {} ({:.2}%)", best_hand.hand_description, best_hand.results.win_rate);
    println!("Worst hand: {} ({:.2}%)", worst_hand.hand_description, worst_hand.results.win_rate);
}

pub fn export_to_csv(results: &[HandResult], filename: &str) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(filename)?;
    
    // Write CSV header
    writeln!(file, "Rank,Hand,Card1,Card2,Win_Rate,Lose_Rate,Tie_Rate,Wins,Losses,Ties,Total_Games")?;
    
    // Write data rows
    for (rank, result) in results.iter().enumerate() {
        let lose_rate = 100.0 - result.results.win_rate - result.results.tie_rate;
        let losses = result.results.total_games - result.results.wins - result.results.ties;
        
        writeln!(file, "{},{},{},{},{:.4},{:.4},{:.4},{},{},{},{}",
                 rank + 1,
                 result.hand_description,
                 result.hand[0],
                 result.hand[1],
                 result.results.win_rate,
                 lose_rate,
                 result.results.tie_rate,
                 result.results.wins,
                 losses,
                 result.results.ties,
                 result.results.total_games)?;
    }
    
    println!("Results exported to: {}", filename);
    Ok(())
}

pub fn export_summary_to_csv(results: &[HandResult], filename: &str, simulations_per_hand: usize, duration: std::time::Duration) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(filename)?;
    
    // Calculate summary statistics
    let avg_win_rate = results.iter().map(|r| r.results.win_rate).sum::<f64>() / results.len() as f64;
    let best_hand = &results[0];
    let worst_hand = &results[results.len() - 1];
    let total_simulations = results.len() * simulations_per_hand;
    
    // Write summary information
    writeln!(file, "=== Texas Hold'em Monte Carlo Analysis Summary ===")?;
    writeln!(file, "Timestamp,{}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"))?;
    writeln!(file, "Total_Hands,{}", results.len())?;
    writeln!(file, "Simulations_Per_Hand,{}", simulations_per_hand)?;
    writeln!(file, "Total_Simulations,{}", total_simulations)?;
    writeln!(file, "Execution_Time_Seconds,{:.2}", duration.as_secs_f64())?;
    writeln!(file, "Simulations_Per_Second,{:.0}", total_simulations as f64 / duration.as_secs_f64())?;
    writeln!(file, "")?;
    writeln!(file, "Average_Win_Rate,{:.4}", avg_win_rate)?;
    writeln!(file, "Best_Hand,{}", best_hand.hand_description)?;
    writeln!(file, "Best_Hand_Win_Rate,{:.4}", best_hand.results.win_rate)?;
    writeln!(file, "Worst_Hand,{}", worst_hand.hand_description)?;
    writeln!(file, "Worst_Hand_Win_Rate,{:.4}", worst_hand.results.win_rate)?;
    
    println!("Summary exported to: {}", filename);
    Ok(())
}

