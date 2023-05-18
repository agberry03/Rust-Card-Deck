use deckofcards::*; // Import everything from the deckofcards crate in Cargo.toml dependencies.
use std::collections::HashMap;
use std::io;
use std::process::exit;

// Global variables.
const BLACK_JACK: usize = 21;

fn main() {
    // Create a new deck.
    let mut deck = Deck::new();
    // Shuffle the deck.
    deck.shuffle();

    // *** Playing Black Jack *** //

    // Initialize player hands with two cards.
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
        score1 = get_score(hand1.clone(), score1);
        show_status(&hand1, score1);
        if is_game_over(score1) {
            exit(0);
        }
        get_hit(&mut hand1, &mut deck);
    }

    print!("\n");

    // Show player 2's deck and score, check if game is won.
    println!("Player 2:");
    while !is_game_over(score2) {
        score2 = get_score(hand2.clone(), score2);
        show_status(&hand2, score2);
        if is_game_over(score2) {
            exit(0);
        }
        get_hit(&mut hand2, &mut deck);
    }
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

// Return the player's current score.
fn get_score(hand: Hand, mut score: usize) -> usize {
    let mut points: usize;
    score = 0;
    for card in hand.cards() {
        if card.rank == Rank::Ace {
            if (score + 11) < BLACK_JACK + 1 {
                points = 11;
            } else {
                points = 1;
            }
        } else if card.rank == Rank::Jack || card.rank == Rank::Queen || card.rank == Rank::King {
            points = 10;
        } else {
            points = card.rank.ordinal() + 2;
        }
        score += points;
    }
    return score;
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
