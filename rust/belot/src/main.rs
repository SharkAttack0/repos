#![allow(unused)]

use core::panic;
use std::default;
use std::io;
use std::mem::swap;
use std::mem::take;
use std::usize;

use rand::seq::SliceRandom;
use strum::*;

use crate::CardSuits::*;
use crate::CardValue::*;
use crate::GameMode::*;

const FIRST_CARD_DEALING_NUM: usize = 5;
const SECOND_CARD_DEALING_NUM: usize = 3;
const NO_TRUMP_ORDER: [CardValue; 8] = [Seven, Eight, Nine, Jack, Queen, King, Ten, Ace];
const TRUMP_ORDER: [CardValue; 8] = [Seven, Eight, Queen, King, Ten, Ace, Nine, Jack];
const REGULAR_ORDER: [CardValue; 8] = [Seven, Eight, Nine, Ten, Jack, Queen, King, Ace];

//can create and shuffle deck
//can print all or one hand
//can add cards to hands
//can compare cards based on the game mode
//can check for cards in a row
//can check for carre (4 cards of equal value)
//
//TO BE DONE:
//user input
//player turns:
//calls pre-game
//playing a card on table (only when allowed)
//restarting the game when everyone has passed
//printing the table (cards in play)
//belot check
//point system
//and more...

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn in_a_row() {
        let result = vec![
            Card {
                value: Seven,
                suit: Clubs,
            },
            Card {
                value: Eight,
                suit: Clubs,
            },
            Card {
                value: Nine,
                suit: Clubs,
            },
            Card {
                value: King,
                suit: Diamonds,
            },
            Card {
                value: Ace,
                suit: Spades,
            },
            Card {
                value: King,
                suit: Spades,
            },
            Card {
                value: Queen,
                suit: Spades,
            },
            Card {
                value: Jack,
                suit: Spades,
            },
        ];
        assert_eq!((), check_cards_sequence(result));
    }
}
fn main() {
    let (mut hands, mut deck) = Hands::new_game();
    hands = Hands::continue_game(hands, &mut deck);
    let input = user_input();
    println!("Before taking card: ");
    print_hand(&hands.p1);
    let mut cards_in_play = Vec::new();
    print_cards_in_play(&cards_in_play, 0);
}

fn run() {}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = String::from(input.trim());
    println!("{input}");
    input
}

fn print_cards_in_play(cards_in_play: &Vec<Card>, mut first_card_index: usize) {
    //fix this
    //prints cards in play (cards can be 0-4)
    //prints position of cards depending on who played first
    //shifts first card by first_card_index positions
    //0 3 2 1
    //1 0 3 2
    //2 1 0 3
    //3 2 1 0
    //0 1 2 3
    //0 3 2 1

    for index in 0..cards_in_play.len() {
        let i = (index + first_card_index) % cards_in_play.len();
        println!(
            "\t\t\tp{}:{:?} {:?}",
            index, cards_in_play[i].value, cards_in_play[i].suit
        );
    }
}

fn take_card(hand: &mut Vec<Card>, index: usize) -> Card {
    if index >= hand.len() {
        println!("Index is bigger than size of hand");
        println!("taking out last card...");
        hand.remove(hand.len() - 1)
    } else {
        hand.remove(index)
    }
}

fn print_hand(hand: &Vec<Card>) {
    for (index, card) in hand.iter().enumerate() {
        println!("{}:\t{:?}\t{:?}", index + 1, card.value, card.suit);
    }
}

fn cards_actual_value(hand: &Vec<Card>, sort_way: [CardValue; 8]) -> Vec<usize> {
    let mut cards_actual_value: Vec<usize> = Vec::new();
    for card in hand.iter() {
        cards_actual_value.push(sort_way.iter().position(|&r| r == card.value).unwrap());
    }
    cards_actual_value
}
fn check_cards_sequence(hand: Vec<Card>) {
    //sorts hand, checks for cards in a row of same suit
    //DOESN'T WORK IN 1 CASE - IN CASE OF 2 SEQUENCES IN
    //SAME SUIT, WILL REGISTER ONLY 1 (excluding cases of quinte)
    let sort_way = REGULAR_ORDER;
    let hand = sort_hand(hand, sort_way);
    let cards_actual_value = cards_actual_value(&hand, sort_way);
    for spec_suit in CardSuits::iter() {
        let mut row_value: usize = 1;
        let mut temp_row_value: usize = 1;
        for index in 0..hand.len() - 1 {
            if hand[index].suit == spec_suit && hand[index + 1].suit == spec_suit {
                if cards_actual_value[index] == cards_actual_value[index + 1] - 1 {
                    temp_row_value += 1;
                } else {
                    row_value = temp_row_value;
                    temp_row_value = 1;
                }
            }
        }

        match row_value {
            3 => println!("\tThis hand has tierce"),
            4 => println!("\tThis hand has a quarte"),
            5 => println!("\tWOW! This hand has a quinte"),
            6 => println!("\tWOW! This hand has a quinte"),
            7 => println!("\tWOW! This hand has a quinte"),
            8 => println!("\tWHAAT! 8 in a row! This hand has a Quinta AND a Terza"),
            _ => {}
        }
    }
}

fn check_carre(hand: &Vec<Card>) {
    let mut value_times = [0, 0, 0, 0, 0, 0];
    for card in hand {
        match card.value {
            Seven => (),
            Eight => (),
            Nine => value_times[0] += 1,
            Ten => value_times[1] += 1,
            Jack => value_times[2] += 1,
            Queen => value_times[3] += 1,
            King => value_times[4] += 1,
            Ace => value_times[5] += 1,
        }
    }
    for (index, val_time) in value_times.iter().enumerate() {
        if *val_time == 4 {
            match index {
                0 => println!("\tThis hand has a carré of Nines!"),
                1 => println!("\tThis hand has a carré of Tens!"),
                2 => println!("\tThis hand has a carré of Jacks!"),
                3 => println!("\tThis hand has a carré of Queens!"),
                4 => println!("\tThis hand has a carré of Kings!"),
                5 => println!("\tThis hand has a carré of Aces!"),
                _ => panic!("At check_carre() out of bounds index"),
            }
        }
    }
}

fn sort_hand(mut hand: Vec<Card>, sort_way: [CardValue; 8]) -> Vec<Card> {
    //takes hand, returns sorted (by suit, by value, weakest to strongest)
    let mut sorted_hand: Vec<Card> = Vec::new();
    let mut card_regular_value: Vec<usize> = cards_actual_value(&hand, sort_way);

    for index in 0..hand.len() {
        let mut smallest_card = card_regular_value[index];
        let mut temp_j = index;
        for j in index + 1..hand.len() {
            if smallest_card > card_regular_value[j] {
                smallest_card = card_regular_value[j];

                temp_j = j;
            }
        }
        let mut temp = hand[index];
        hand[index] = hand[temp_j];
        hand[temp_j] = temp;
        let mut temp_card_regular_value = card_regular_value[index];
        card_regular_value[index] = card_regular_value[temp_j];
        card_regular_value[temp_j] = temp_card_regular_value;
    }

    let mut sorted_suit_hand: Vec<Card> = Vec::new();

    for spec_suit in CardSuits::iter() {
        for card in &hand {
            if card.suit != spec_suit {
                continue;
            }
            sorted_suit_hand.push(*card);
        }
    }
    sorted_suit_hand
}

fn cards_compare(cards_in_play: Vec<Card>, game_mode: &GameMode) -> Card {
    //takes vec of n Cards and game_mode, returns strongest
    let mut card_strongest_index: usize = 0;
    let mut card_num: Vec<usize> = Vec::new();
    let mut trump_suit: CardSuits = Clubs;
    let mut one_trump = false;
    match game_mode {
        OneTrump(trump) => {
            trump_suit = *trump;
            one_trump = true;
        }
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
                    card_num.push(
                        NO_TRUMP_ORDER
                            .iter()
                            .position(|&r| r == card.value)
                            .unwrap(),
                    );
                }
            }
            NoTrumps => card_num.push(
                NO_TRUMP_ORDER
                    .iter()
                    .position(|&r| r == card.value)
                    .unwrap(),
            ),
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
            }
            if trump_played == true && card.suit != trump_suit {
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

        self.p1 = sort_hand(self.p1, REGULAR_ORDER);
        self.p2 = sort_hand(self.p2, REGULAR_ORDER);
        self.p3 = sort_hand(self.p3, REGULAR_ORDER);
        self.p4 = sort_hand(self.p4, REGULAR_ORDER);

        self
    }
    fn iter(&self) -> Vec<&Vec<Card>> {
        vec![&self.p1, &self.p2, &self.p3, &self.p4]
    }
    fn iter_mut(&mut self) -> Vec<&mut Vec<Card>> {
        vec![&mut self.p1, &mut self.p2, &mut self.p3, &mut self.p4]
    }
    fn print_all_hands(&self) {
        for (index, hand) in self.iter().iter().enumerate() {
            println!("Hand #{}", index + 1);
            for card in hand.iter() {
                println!("\t{:?}\t{:?}", card.value, card.suit);
            }
            check_carre(hand);
            check_cards_sequence(hand.to_vec());
            println!();
        }
    }
    fn print_hand(&self, index: usize) {
        println!("Hand #{}", index + 1);
        let hand = self.iter()[index];
        for card in hand.iter() {
            println!("\t{:?}\t{:?}", card.value, card.suit);
        }
        println!();
    }

    fn new_game() -> (Self, Vec<Card>) {
        //creates new empty hands
        //creates new shuffled deck
        //calls add_cards with FIRST_CARD_DEALING_NUM
        //prints hands and returns
        let mut hands = Hands::new();
        let mut deck = generate_full_deck();
        hands = Hands::add_cards(hands, &mut deck, FIRST_CARD_DEALING_NUM);
        println!("\nStarting a new game!");
        println!("Added 5 cards to each hand:\n");
        Hands::print_all_hands(&hands);
        (hands, deck)
    }
    fn continue_game(mut self, deck: &mut Vec<Card>) -> Self {
        //adds 3 cards to each hand, prints and returns
        self = Hands::add_cards(self, deck, SECOND_CARD_DEALING_NUM);
        for (index, hand) in self.iter().iter().enumerate() {
            for card in hand.iter() {}
        }
        println!("\nContinuing game!");
        println!("Added 3 more cards to each hand. Good luck!\n");
        Hands::print_all_hands(&self);
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
