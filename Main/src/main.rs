use deckofcards::*; // Import everything from the deckofcards crate in Cargo.toml dependencies.
use std::io; // Import input and output.
use std::process::exit; // Import the exit method.

// Global variables.
const BLACK_JACK: usize = 21;

fn main() {
    // Create a new deck.
    let mut deck = Deck::new(); // Creates a standard 52 card deck.
    // Shuffle the deck.
    deck.shuffle();

    // *** Playing Black Jack *** //

    // Initialize player hands. Deal two cards to each hand.
    let mut hand1 = Hand::new();
    deck.deal_to_hand(&mut hand1, 2);
    let mut hand2 = Hand::new();
    deck.deal_to_hand(&mut hand2, 2);

    // Initialize player scores.
    let mut score1 = 0;
    let mut score2 = 0;

    // Show player 1's deck and score, check if game is won.
    println!("Player 1:");
    while !is_game_over(score1) {
        // Update score.
        score1 = get_score(hand1.clone(), score1);
        // Show cards in deck and current score.
        show_status(&hand1, score1);
        // End the program if the player gets a score of 21+.
        if is_game_over(score1) {
            exit(0);
        }
        // Ask if player would like to hit or pass turn and act accordingly.
        if hit_or_pass() {
            get_hit(&mut hand1, &mut deck);
        } else {
            break;
        }
    }

    print!("\n");

    // Show player 2's deck and score, check if game is won, get a hit or pass.
    // Very similar to previous code. I may try to create a method for it eventually.
    println!("Player 2:");
    while !is_game_over(score2) {
        score2 = get_score(hand2.clone(), score2);
        show_status(&hand2, score2);
        if is_game_over(score2) {
            exit(0);
        }
        if hit_or_pass() {
            get_hit(&mut hand2, &mut deck);
        } else {
            break;
        }
    }

    print!("\n");

    // If no winner has been determined yet, compare the players final scores.
    // The score closer to 21 wins.
    compare_scores(score1, score2);
}

// Display the player's current hand and score.
fn show_status(hand: &Hand, score: usize) {
    print!("Cards : ");
    for card in hand.cards() {
        print!("{}, ", card);
    }
    print!("\n");
    println!("Current score : {}", score);
}

// Calculate and return the player's current score.
fn get_score(hand: Hand, mut score: usize) -> usize {
    let mut points: usize;
    // Reset the score to prevent previously drawn card's values from doubling on.
    score = 0;
    for card in hand.cards() {
        // Point logic for ace cards.
        if card.rank == Rank::Ace {
            if (score + 11) < BLACK_JACK + 1 {
                points = 11;
            } else {
                points = 1;
            }
        // Point logic for face cards.
        } else if card.rank == Rank::Jack || card.rank == Rank::Queen || card.rank == Rank::King {
            points = 10;
        // Point logic for all other cards.
        } else {
            points = card.rank.ordinal() + 2; // The ordinal number needs to be incremented by 2 for accuracy.
        }
        score += points;
    }
    return score;
}

// Determine if the current player will hit for a new draw or pass their turn.
fn hit_or_pass() -> bool {
    println!("Enter '1' to take a hit, or '2' to pass:");
    let mut answer = String::new();
    let mut hit: bool = false;

    while answer.trim() != "1" || answer.trim() != "2" {
        io::stdin()
            .read_line(&mut answer)
            .expect("Could not read input.");

        if answer.trim() == "1" {
            hit = true;
            break;
        } else if answer.trim() == "2" {
            hit = false;
            break;
        } else {
            println!("Please enter 1 or 2:");
        }
    }
    return hit;
}

// Draw a new card into the hand.
fn get_hit(hand: &mut Hand, deck: &mut Deck) {
    deck.deal_to_hand(hand, 1);
}

// Check if the game is over. If the game is not over, return false.
fn is_game_over(score: usize) -> bool {
    if score == BLACK_JACK {
        println!("Black Jack! You win!");
        return true;
    } else if score > BLACK_JACK {
        println!("Your score's too high. You lose!");
        return true;
    }
    return false;
}

// If both players pass without winning or losing, then compare their final scores.
fn compare_scores(score1: usize, score2: usize) {
    if score1 > score2 {
        println!("Player 1 won!");
    } else if score1 < score2 {
        println!("Player 2 won!");
    } else {
        println!("It's a tie!");
    }
}
