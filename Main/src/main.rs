use std::io;
use std::collections::HashMap;
use deckofcards::*;

fn main() {
    // Create a new deck.
    let mut deck = Deck::new();
    // Shuffle the deck.
    deck.shuffle();

    // Deal a single card out.
    let deck = deck.deal(1);

    
    for card in deck.iter() {
        println!("Cards : {}", card);
    }
}