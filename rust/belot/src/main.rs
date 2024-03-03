#![allow(unused)]

use crate::CardSuits::*;
use crate::CardValue::*;
use crate::GameMode::*;
use std::fmt::write;
use std::num::NonZeroI128;
use std::default;
use std::process::Termination;
use rand::seq::SliceRandom;
use strum::*;
use core::num;

const FIRST_CARD_DEALING_NUM: usize = 5;
const SECOND_CARD_DEALING_NUM: usize = 3;
const NO_TRUMP_ORDER: [CardValue; 8] = [Seven, Eight, Nine, Jack, Queen, King, Ten, Ace];
const TRUMP_ORDER: [CardValue; 8] = [Seven, Eight, Queen, King, Ten, Ace, Nine, Jack];

fn main() {
    let (mut hands, mut deck) = Hands::new_game();
    hands = Hands::continue_game(hands, &mut deck);
    let foo = Card {
        value: Ace,
        suit: Spades,
    };
    let soo = Card {
        value: Jack,
        suit: Diamonds,
    };
    let goo = Card {
        value: Nine,
        suit: Spades,
    };
    let doo = Card {
        value: Seven,
        suit: Clubs,
    };
    
    let game_mode: GameMode  = OneTrump(Diamonds);

    let mut cards_in_play: Vec<Card> = Vec::with_capacity(4);
    cards_in_play.push(foo);
    cards_in_play.push(goo);
    cards_in_play.push(doo);
    cards_in_play.push(soo);

    let card_strongest= cards_compare(cards_in_play, &game_mode);

    println!("Strongest card is the {:?} of {:?}", card_strongest.value, card_strongest.suit);
}

fn cards_compare(cards_in_play: Vec<Card>, game_mode: &GameMode) -> Card {
    //takes vec of n Cards and game_mode, returns strongest 
    let mut card_strongest_index: usize = 0;
    let mut card_num: Vec<usize> = Vec::new();
    let mut trump_suit = CardSuits::Clubs;
    let mut one_trump = false;
    match game_mode {
        OneTrump(trump) => {
            trump_suit = *trump;
            one_trump = true;
        },
        NoTrumps => (),
        AllTrumps => (),
    };
    //create vec with value of each card (value depending if its trump)
    for card in cards_in_play.iter() {
        match game_mode {
            OneTrump(_) => {
                if card.suit == trump_suit {
                    card_num.push(TRUMP_ORDER.iter().position(|&r| r == card.value).unwrap());
                } else {
                    card_num.push(NO_TRUMP_ORDER.iter().position(|&r| r == card.value).unwrap());
                }
            },
            NoTrumps => card_num.push(NO_TRUMP_ORDER.iter().position(|&r| r == card.value).unwrap()),
            AllTrumps => card_num.push(TRUMP_ORDER.iter().position(|&r| r == card.value).unwrap()),
        };
    }
    let mut temp_card_strongest_value = card_num[0];
    let mut trump_played = false;
    //compare each card, in one trump case compare only when needed (otherwise incorrect result)
    for (index, card) in cards_in_play.iter().enumerate() {
        if one_trump {
            if trump_played == false && card.suit == trump_suit {
                //case 2 - No trumps played, current card is trump, DON'T compare
                trump_played = true;
                temp_card_strongest_value = card_num[index];
                card_strongest_index = index;
                continue;
            } else if trump_played == true && card.suit != trump_suit {
                //case 3 - Trump played, current card non-trump, DON'T compare
                continue;
            }
        }
        if temp_card_strongest_value < card_num[index] {
            temp_card_strongest_value = card_num[index];
            card_strongest_index = index;
        }    
    }
    cards_in_play[card_strongest_index]
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
        //adds certain amount of cards to each hand and returns hands
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
    fn iter(&self) -> Vec<&Vec<Card>> {
        vec![&self.p1, &self.p2, &self.p3, &self.p4]
    }
    fn print_hands(&self) {
        for (index, hand) in self.iter().iter().enumerate() { //yea... fix that shi
            println!("Hand #{}", index + 1);
            for card in hand.iter() {
                println!("\t{:?}\t{:?}", card.value, card.suit);
            }
            println!();
        }
    }
    fn new_game() -> (Hands, Vec<Card>) {
        //creates new empty hands
        //creates new shuffled deck
        //calls add_cards with FIRST_CARD_DEALING_NUM
        //prints hands and returns
        let mut hands = Hands::new();
        let mut deck = generate_full_deck();
        hands = Hands::add_cards(hands, &mut deck, FIRST_CARD_DEALING_NUM);
        println!("\nStarting a new game!");
        println!("Added 5 cards to each hand:\n");
        Hands::print_hands(&hands);
        (hands, deck)
    }
    fn continue_game(mut self, deck: &mut Vec<Card>) -> Self {
        //adds 3 cards to each hand, prints and returns
        self = Hands::add_cards(self, deck, SECOND_CARD_DEALING_NUM);
        for (index, hand) in self.iter().iter().enumerate() {
            for card in hand.iter() {
                
            }
        } 
        println!("\nContinuing game!");
        println!("Added 3 more cards to each hand. Good luck!\n");
        Hands::print_hands(&self);
        self
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

#[derive(Debug, PartialEq)]
enum GameMode {
    OneTrump(CardSuits),
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
