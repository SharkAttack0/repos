#![allow(unused)]

use std::default;
use std::fmt::write;
use std::io;
use std::iter;
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
//can take user input
//can evaluate point system from cards
//can print the table (cards in play)
//can add points from announcments
//
//TO BE DONE:
//player turns:
//bidding
//playing a card on table (only when allowed)
//restarting the game when everyone has passed
//belot check
//and more...
//
//MAJOR problems;
//Bidding
//init player after every turn

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
    }
}
fn main() {
    run();
}
//order of events:
//1 - new deck, new hands, each of 5 cards
//2 - bidding, restart if 4 passes !!!
//3 - add 3 cards to each hand
//4 - start of game, first player plays card
//5 - all other players respond, strongest card takes
//6 - repeat until cards are over
//7 - 2 vecs - of each team, count points
//8 - add points to each team
//9 - repeat with next player starting, until team has >=151 points
//10 - repeat
fn run() {
    loop {
        let (mut hands, mut deck) = new_game();
        let game_mode: GameMode = AllTrumps;
        let mut points_from_announs: [usize; 2] = [0, 0];
        hands = continue_game(hands, &mut deck, &game_mode, &mut points_from_announs);
        let mut cards_in_play: Vec<Card> = Vec::with_capacity(4);
        let mut point_decks: [Vec<Card>; 2] = [vec![], vec![]];
        for card_index in 0..hands[0].len() {
            for hand_index in 0..4 {
                print_cards_in_play(&cards_in_play, 0);
                println!("player #{}", hand_index + 1);
                cards_in_play.push(ask_play_card(&mut hands[hand_index]));
            }
            print_cards_in_play(&cards_in_play, 0);
            //win_hand_index is the same index as first player of next turn
            let win_hand_index = cards_compare(&cards_in_play, &game_mode);
            //assume indexes 0 and 2 are of one team, as well as 1 and 3
            if win_hand_index % 2 == 0 {
                point_decks[0].append(&mut cards_in_play);
            } else {
                point_decks[1].append(&mut cards_in_play);
            }
        }

        let mut points_game: [usize; 2] = point_count(point_decks, &game_mode);

        for index in 0..2 {
            println!(
                "Team #{}'s points from announcments: {}",
                index + 1,
                points_from_announs[index]
            );
            points_game[index] += points_from_announs[index];
            println!(
                "Team #{}'s points from game: {}",
                index + 1,
                points_game[index]
            );
        }

        break;
    }
}

fn check_cards_sequence(hand: &Vec<Card>, hand_index: usize, points_count: &mut [usize; 2]) {
    //sorts hand, checks for cards in a row of same suit
    //DOESN'T WORK IN 1 CASE - IN CASE OF 2 SEQUENCES IN
    //SAME SUIT, WILL REGISTER ONLY 1 (excluding cases of quinte)
    let sort_way = REGULAR_ORDER;
    let hand = sort_hand(&mut hand.clone(), sort_way);
    let cards_actual_value = cards_actual_value(&hand, sort_way);
    let mut row_value: usize = 1;
    for spec_suit in CardSuits::iter() {
        row_value = 1;
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
            3 => {
                println!("\tThis hand has tierce\n");
                points_count[hand_index % 2] += 20;
            }
            4 => {
                println!("\tThis hand has a quarte\n");
                points_count[hand_index % 2] += 50;
            }
            5 => {
                println!("\tWOW! This hand has a quinte\n");
                points_count[hand_index % 2] += 100;
            }
            6 => {
                println!("\tWOW! This hand has a quinte\n");
                points_count[hand_index % 2] += 100;
            }
            7 => {
                println!("\tWOW! This hand has a quinte\n");
                points_count[hand_index % 2] += 100;
            }
            8 => {
                println!("\tWOW! This hand has a quinte\n");
                points_count[hand_index % 2] += 100;
            }
            _ => (),
        }
    }
}

fn check_carre(hand: &Vec<Card>, hand_index: usize, points_count: &mut [usize; 2]) {
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
                0 => {
                    println!("\tThis hand has a carré of Nines!");
                    points_count[hand_index % 2] += 150;
                }
                1 => {
                    println!("\tThis hand has a carré of Tens!");
                    points_count[hand_index % 2] += 100;
                }
                2 => {
                    println!("\tThis hand has a carré of Jacks!");
                    points_count[hand_index % 2] += 200;
                }
                3 => {
                    println!("\tThis hand has a carré of Queens!");
                    points_count[hand_index % 2] += 100;
                }
                4 => {
                    println!("\tThis hand has a carré of Kings!");
                    points_count[hand_index % 2] += 100;
                }
                5 => {
                    println!("\tThis hand has a carré of Aces!");
                    points_count[hand_index % 2] += 100;
                }
                _ => panic!("At check_carre() out of bounds index"),
            };
        }
    }
}
fn point_count(point_decks: [Vec<Card>; 2], game_mode: &GameMode) -> [usize; 2] {
    //takes 2 decks and transforms cards into points for each team
    //
    //TO BE DONE:
    //  1:  take extra parameter of type [usize;2] which is total points
    //      from announcments for each team and adds it to the points_total at the end
    //  2:  Add or remove points based on capot, vutrene and others
    //      both additions could be in their own function
    //
    let mut points_total: [usize; 2] = [0, 0];
    for index in 0..2 {
        for card in point_decks[index].iter() {
            let points_order = match game_mode {
                NoTrumps => PointsOrder::NoTrumps,
                AllTrumps => PointsOrder::AllTrumps,
                OneTrump(trump_suit) => {
                    if card.suit == *trump_suit {
                        PointsOrder::AllTrumps
                    } else {
                        PointsOrder::NoTrumps
                    }
                }
                Pass => panic!("Pass is not supposed to be possible here (point_count())"),
            };

            //This is 1 of 2 ways to do it:
            //2nd way is to make arrays of each team's points, then add them
            //(this is in the case that you need arrays of points for some reason)
            points_total[index] += match points_order {
                PointsOrder::NoTrumps => match card.value {
                    Seven => 0,
                    Eight => 0,
                    Nine => 0,
                    Jack => 2,
                    Queen => 3,
                    King => 4,
                    Ten => 10,
                    Ace => 11,
                },
                PointsOrder::AllTrumps => match card.value {
                    Seven => 0,
                    Eight => 0,
                    Queen => 3,
                    King => 4,
                    Ten => 10,
                    Ace => 11,
                    Nine => 14,
                    Jack => 20,
                },
            }
        }
    }

    //in case of No trumps: multiply points by 2
    match game_mode {
        NoTrumps => {
            points_total[0] *= 2;
            points_total[1] *= 2;
        }
        _ => (),
    }
    points_total
}

fn points_from_sequence(row_value: usize) -> usize {
    match row_value {
        3 => 20,         //Terza
        4 => 50,         //Quarte
        5 => 100,        //Quinta
        6 => 100,        //Quinta
        7 => 100,        //Quinta
        8 => (100 + 20), //Quinta + Terza
        _ => panic!("row_value out of bounds at points_from_sequence()"),
    }
}

enum PointsOrder {
    NoTrumps,
    AllTrumps,
}
//bidding - init player, calls,
//next player - repeat
//if raising bid is possible, ask each player again
//on 3 passes in a row, start game, on 4 passes - restart
fn bidding(hands: [Vec<Card>; 4], cur_highest_bid: GameMode) -> usize {
    let mut ans_int: usize = 1;
    let mut skip_bids;
    match cur_highest_bid {
        Pass => skip_bids = 0,
        OneTrump(Clubs) => skip_bids = 1,
        OneTrump(Diamonds) => skip_bids = 2,
        OneTrump(Hearts) => skip_bids = 3,
        OneTrump(Spades) => skip_bids = 4,
        NoTrumps => skip_bids = 5,
        AllTrumps => skip_bids = 6,
    }
    let game_mode_count = 7; //uch...
                             //
    let mut bid;
    match ans_int {
        1 => bid = Pass,
        2 => bid = AllTrumps,
        3 => bid = NoTrumps,
        4 => bid = OneTrump(Spades),
        5 => bid = OneTrump(Hearts),
        6 => bid = OneTrump(Diamonds),
        7 => bid = OneTrump(Clubs),
        _ => panic!("ans_int at bidding()"),
    }
    ans_int
}

fn print_bidding_options(cur_highest_bid: GameMode) {
    println!("{:?}", Pass);
    println!("{:?}", AllTrumps);
    println!("{:?}", NoTrumps);
    println!("{:?}", OneTrump(Spades));
    println!("{:?}", OneTrump(Hearts));
    println!("{:?}", OneTrump(Diamonds));
    println!("{:?}", OneTrump(Clubs));
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input = String::from(input.trim());
    input
}

fn user_input_to_int(max_allowed_int: usize) -> usize {
    let mut input_int;
    loop {
        let input = user_input();
        let input_to_int = input.parse::<usize>();
        match input_to_int {
            Ok(int) => input_int = int,
            Err(_) => {
                println!("Error: invalid input!");
                continue;
            }
        }
        if input_int == 0 || input_int > max_allowed_int {
            println!("Error: invalid number!");
            continue;
        }
        break;
    }
    //subtract one so that input represents actual order #
    input_int - 1
}

fn ask_play_card(hand: &mut Vec<Card>) -> Card {
    let mut ans_int;
    println!("Choose a card:");
    print_hand(hand, true);
    ans_int = user_input_to_int(hand.len());
    take_card(hand, ans_int)
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
            index + 1,
            cards_in_play[i].value,
            cards_in_play[i].suit
        );
    }
    println!();
}

fn take_card(hand: &mut Vec<Card>, index: usize) -> Card {
    hand.remove(index)
}

fn print_hand(hand: &Vec<Card>, label_card: bool) {
    for (index, card) in hand.iter().enumerate() {
        if label_card {
            println!("{}:\t{:?}\t{:?}", index + 1, card.value, card.suit);
        } else {
            println!("\t{:?}\t{:?}", card.value, card.suit);
        }
    }
    println!();
}

fn cards_actual_value(hand: &Vec<Card>, sort_way: [CardValue; 8]) -> Vec<usize> {
    //returns ints of cards' values according to a specified ordering
    let mut cards_actual_value: Vec<usize> = Vec::new();
    for card in hand.iter() {
        cards_actual_value.push(sort_way.iter().position(|&r| r == card.value).unwrap());
    }
    cards_actual_value
}

fn sort_hand(hand: &mut Vec<Card>, sort_way: [CardValue; 8]) -> Vec<Card> {
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
        for card in hand.iter() {
            if card.suit != spec_suit {
                continue;
            }
            sorted_suit_hand.push(*card);
        }
    }
    sorted_suit_hand
}

fn cards_compare(cards_in_play: &Vec<Card>, game_mode: &GameMode) -> usize {
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
        Pass => panic!("Pass variant is not supposed to be possbible here!"),
        _ => (),
    };
    //create vec with value of each card (value depending if its trump)
    for card in cards_in_play.iter() {
        match game_mode {
            Pass => panic!("Pass variant is not supposed to be possbible here!"),
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
    let mut init_suit = cards_in_play[0].suit;
    //compare each card, in one trump case compare only when needed (otherwise incorrect result)
    for (index, card) in cards_in_play.iter().enumerate() {
        if one_trump {
            if trump_played == false && card.suit == trump_suit {
                //case 2 - No trumps played, current card is trump, DON'T compare
                trump_played = true;
                temp_card_strongest_value = card_num[index];
                card_strongest_index = index;
                init_suit = trump_suit;
                continue;
            }
            if trump_played == true && card.suit != trump_suit {
                //case 3 - Trump played, current card non-trump, DON'T compare
                continue;
            }
        }

        if card.suit == init_suit {
            if temp_card_strongest_value < card_num[index] {
                temp_card_strongest_value = card_num[index];
                card_strongest_index = index;
            }
        }
    }
    println!(
        "\tThe strongest card is the {:?} of {:?}",
        cards_in_play[card_strongest_index].value, cards_in_play[card_strongest_index].suit
    );
    card_strongest_index
}

fn new_hands() -> [Vec<Card>; 4] {
    [vec![], vec![], vec![], vec![]]
}

fn add_cards(mut hands: [Vec<Card>; 4], deck: &mut Vec<Card>, num_add: usize) -> [Vec<Card>; 4] {
    //adds certain amount of cards to each hand and returns hands

    for index in 0..hands.len() {
        hands[index].extend(deck.iter().take(num_add));
        deck.drain(..num_add);
    }
    hands
}

fn new_game() -> ([Vec<Card>; 4], Vec<Card>) {
    //creates new empty hands
    //creates new shuffled deck
    //calls add_cards with FIRST_CARD_DEALING_NUM
    //prints hands and returns
    let mut hands = new_hands();
    let mut deck = generate_full_deck();
    hands = add_cards(hands, &mut deck, FIRST_CARD_DEALING_NUM);

    println!("\nStarting a new game!");
    println!("Added 5 cards to each hand:\n");
    for index in 0..4 {
        hands[index] = sort_hand(&mut hands[index], NO_TRUMP_ORDER);
        print_hand(&hands[index], false);
    }
    (hands, deck)
}
fn continue_game(
    mut hands: [Vec<Card>; 4],
    deck: &mut Vec<Card>,
    game_mode: &GameMode,
    points_from_announs: &mut [usize; 2],
) -> [Vec<Card>; 4] {
    //adds 3 cards to each hand, prints and returns
    hands = add_cards(hands, deck, SECOND_CARD_DEALING_NUM);
    println!("\nContinuing game!");
    println!("Added 3 more cards to each hand. Good luck!\n");
    for index in 0..4 {
        match game_mode {
            NoTrumps => hands[index] = sort_hand(&mut hands[index], NO_TRUMP_ORDER),
            AllTrumps => hands[index] = sort_hand(&mut hands[index], TRUMP_ORDER),
            OneTrump(trump_suit) => hands[index] = sort_hand(&mut hands[index], NO_TRUMP_ORDER),
            Pass => (),
        }
        println!("player #{}:", index + 1);
        print_hand(&hands[index], false);
        check_carre(&hands[index], index, points_from_announs);
        check_cards_sequence(&hands[index], index, points_from_announs);
    }
    hands
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

#[derive(Debug, PartialEq, EnumIter, Copy, Clone)]
enum Bidding {
    Pass,
    GameMode(GameMode),
}

#[derive(Debug, PartialEq, EnumIter, Clone, Copy, Default)]
enum GameMode {
    Pass,
    OneTrump(CardSuits),
    #[default]
    NoTrumps,
    AllTrumps,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    value: CardValue,
    suit: CardSuits,
}

#[derive(Debug, PartialEq, EnumIter, Copy, Clone, Default)]
pub enum CardSuits {
    #[default]
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
