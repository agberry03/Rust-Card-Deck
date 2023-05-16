use deckofcards::*; // Import everything from the deckofcards crate in Cargo.toml dependencies.
use std::io;

// Global variables.
const BLACK_JACK: usize = 21;

fn main() {
    // Create a new deck.
    let mut deck = Deck::new();
    // Shuffle the deck.
    deck.shuffle();

    // *** Playing Black Jack *** //

    // Initialize player hands with two cards.
    let hand1 = deck.deal(2);
    let hand2 = deck.deal(2);

    // Initialize player scores.
    let mut score1 = 0;
    let mut score2 = 0;

    // Show player 1's deck and score, check if game is won.
    println!("Player 1:");
    score1 = get_score(hand1.clone(), score1);
    show_status(hand1, score1);
    is_game_over(score1);
    print!("\n");

    // Show player 2's deck and score, check if game is won.
    println!("Player 2:");
    score2 = get_score(hand2.clone(), score2);
    show_status(hand2, score2);
    is_game_over(score2);
}

// Display the player's current hand and score.
fn show_status(hand: Vec<Card>, score: usize) {
    print!("Cards : ");
    for card in hand.iter() {
        print!("{}, ", card);
    }
    print!("\n");
    println!("Current score : {}", score);
}

// Return the player's current score.
fn get_score(hand: Vec<Card>, mut score: usize) -> usize {
    for card in hand.iter() {
        if card.rank == Rank::Ace {
            if (score + 11) < BLACK_JACK + 1 {
                score += 11;
            } else {
                score += 1
            }
        } else if card.rank == Rank::Jack || card.rank == Rank::Queen || card.rank == Rank::King {
            score += 10;
        } else {
            score += card.rank.ordinal() + 2;
        }
    }
    return score;
}

fn is_game_over(score: usize) -> bool {
    if score == BLACK_JACK {
        println!("Black Jack! You win!");
        return true;
    } else if score > BLACK_JACK {
        println!("You lose!");
        return true;
    }
    return false;
}
