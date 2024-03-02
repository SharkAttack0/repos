#![allow(unused)]

use crate::CardSuits::*;
use crate::CardValue::*;
use crate::GameMode::*;
use core::num;
use rand::seq::SliceRandom;
use std::default;
use std::process::Termination;
use strum::*;

const FIRST_CARD_DEALING_NUM: usize = 5;
const SECOND_CARD_DEALING_NUM: usize = 3;
const NO_TRUMPS_ORDER: [CardValue; 8] = [Seven, Eight, Nine, Jack, Queen, King, Ten, Ace];
const ALL_TRUMPS_ORDER: [CardValue; 8] = [Seven, Eight, Queen, King, Ten, Ace, Nine, Jack];

fn main() {
    // let (mut hands, mut deck) = Hands::new_game();
    // hands = Hands::continue_game(hands, &mut deck);
    let foo = Card {
        value: Ace,
        suit: Spades,
    };
    let goo = Card {
        value: Nine,
        suit: Clubs,
    };

    let mut gamemode = GameMode::NoTrumps;
    let mut card_order: [CardValue; 8];
    match gamemode {
        TrumpClubs => card_order = NO_TRUMPS_ORDER,
        TrumpDiamonds => card_order = NO_TRUMPS_ORDER,
        TrumpHearts => card_order = NO_TRUMPS_ORDER,
        TrumpSpades => card_order = NO_TRUMPS_ORDER,
        NoTrumps => {
            card_order = NO_TRUMPS_ORDER;
            println!("NoTrumps initiated");
        }
        AllTrumps => card_order = ALL_TRUMPS_ORDER,
    };
    let foo_num = card_order.iter().position(|&r| r == foo.value).unwrap();

    let goo_num = card_order.iter().position(|&r| r == goo.value).unwrap();

    if foo_num > goo_num {
        println!("success!");
    }
}

fn compare_cards(foo: Card, goo: Card) {}

enum GameMode {
    TrumpClubs,
    TrumpDiamonds,
    TrumpHearts,
    TrumpSpades,
    NoTrumps,
    AllTrumps,
}

#[derive(Debug)]
struct Hands {
    p1: Vec<Card>,
    p2: Vec<Card>,
    p3: Vec<Card>,
    p4: Vec<Card>,
}

impl Hands {
    fn new() -> Self {
        Self {
            p1: vec![],
            p2: vec![],
            p3: vec![],
            p4: vec![],
        }
    }
    fn add_cards(mut self, deck: &mut Vec<Card>, num_add: usize) -> Self {
        self.p1.extend(deck.iter().take(num_add));
        deck.drain(..num_add);
        self.p2.extend(deck.iter().take(num_add));
        deck.drain(..num_add);
        self.p3.extend(deck.iter().take(num_add));
        deck.drain(..num_add);
        self.p4.extend(deck.iter().take(num_add));
        deck.drain(..num_add);
        self
    }
    fn print_hands(&self) {
        println!("{self:#?\n}");
    }
    fn new_game() -> (Hands, Vec<Card>) {
        //creates new empty hands
        //creates new shuffled deck
        //gives FIRST_CARD_DEALING_NUM cards to each hand from deck
        //prints hands and returns
        let mut hands = Hands::new();
        let mut deck = generate_full_deck();
        hands = Hands::add_cards(hands, &mut deck, FIRST_CARD_DEALING_NUM);
        Hands::print_hands(&hands);
        (hands, deck)
    }
    fn continue_game(mut self, deck: &mut Vec<Card>) -> Self {
        //adds 3 cards to each hand and returns
        let hands = Hands::add_cards(self, deck, SECOND_CARD_DEALING_NUM);
        Hands::print_hands(&hands);
        hands
    }
}

fn generate_full_deck() -> Vec<Card> {
    //generates a full deck of each unique card
    //puts it in a vector
    //shuffles it and returns it
    let mut deck = vec![];
    for suit in CardSuits::iter() {
        for value in CardValue::iter() {
            deck.push(Card { value, suit });
        }
    }
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    deck
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuits,
}

#[derive(Debug, PartialEq, EnumIter, Copy, Clone)]
pub enum CardSuits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, EnumIter, Copy, Clone, PartialEq)]
pub enum CardValue {
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
