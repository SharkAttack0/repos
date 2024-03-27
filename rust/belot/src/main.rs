#![allow(unused)]
use std::io;
use std::panic;
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

//THIS VERSION FOLLOWS THE belot.bg RULES
//(PLUS OPINIONS/PREFERENCES FROM FRIENDS)

//THINGS TO CLARIFY:
//  N - Null
//  0-hanging points check BEFORE or AFTER rounding
//      0 - Before
//      1 - After
//  1-hanging points whole game or half
//      0 - half (add half to non-declared team)
//      1 - whole
//  2-contra and re-contra
//      0 - Only on AllTrumps
//      1 - Always available
//  3-what can you call after a contra?
//      0 - only re-contra
//      1 - higher bid or re-contra
//
//
//rosen - 0-N, 1-N, 2-0, 3-N,
//ioan - 0-1, 1-1, 2-N, 3-1,
//
//TO BE DONE:
//  bidding's contra
//  test comparing card sequences
//
//REFACTORING
//
//QOF UPDATES:
//  don't print and don't count cards witch are not valid (also required for bots)
//
//MAKE INSTANCE FOR EACH VERSION
//
//BOTS:
//  get vec of legal cards to be played and create condition to decide
//  which one to play (or simply randomize it)
//  get dominant cards and play them first
//  awareness of teammate's changed bid?
//  bots get better cards (niki idea)

fn main() {
    run();
}

fn run() {
    //these are variables that must carry throughout games
    //total points for both teams
    let mut points_total: [usize; 2] = [0, 0];
    //first player who bids and playes first card
    let mut init_hand_index = 0;
    //variable that keeps track of hanging points throughout games
    let mut hanging_points = 0;

    //game loop
    loop {
        //these are variables that are needed for a game
        //most important var - keeps track of trick's winner
        let mut win_hand_index = init_hand_index;
        //points from current game
        let mut points_game: [usize; 2];
        //points from announcments (obsolete for NoTrumps)
        let mut points_from_announs: [usize; 2] = [0, 0];
        //decks of cards formed by winning tricks
        let mut point_decks: [Vec<Card>; 2] = [vec![], vec![]];
        //create first hands and deck
        let (mut hands, mut deck) = new_game();
        //current cards on table
        let mut cards_in_play: Vec<Card> = Vec::with_capacity(4);
        //determine game mode and player who bid (needed for checks at end)
        let (game_mode, player_last_bid_index) = bidding(win_hand_index);
        match game_mode {
            Pass => {
                println!("\nAll players passed! Restarting...\n");
                init_hand_index = (init_hand_index + 1) % 4;
                println!("Enter any key to continue");
                user_input();
                continue;
            }
            _ => println!("\nThe game mode is {:?}\n", game_mode),
        }

        hands = continue_game(hands, &mut deck, &game_mode, &mut points_from_announs);

        //actual playing
        for _ in 0..hands[0].len() {
            //init_card value here should never be of actual use
            let mut init_card: Card;
            //for for current turn
            for hand_index in 0..4 {
                let actual_player_index = (hand_index + win_hand_index) % 4;
                print_cards_in_play(&cards_in_play, win_hand_index);
                println!("player #{}", actual_player_index + 1);
                if hand_index == 0 {
                    //skip card validation if first card
                    let init_card_index = ask_play_card(&mut hands[win_hand_index]);
                    init_card = hands[win_hand_index][init_card_index];
                    hands[win_hand_index].remove(init_card_index);
                    cards_in_play.push(init_card);
                } else {
                    init_card = cards_in_play[cards_compare(&cards_in_play, &game_mode)];
                    cards_in_play.push(card_validation(
                        &mut hands[actual_player_index],
                        &game_mode,
                        init_card,
                        &cards_in_play,
                        win_hand_index,
                    ));
                }
                //belot check
                if belot_check(
                    &hands[actual_player_index],
                    &game_mode,
                    cards_in_play[hand_index],
                    init_card,
                ) {
                    points_from_announs[actual_player_index % 2] += 20;
                }
            }
            print_cards_in_play(&cards_in_play, win_hand_index);
            //win_hand_index is the same index as first player of next turn
            //cards_comapre() returns winning card index, which represents
            //the number of positions from the initial card (win_hand_index)
            win_hand_index = (win_hand_index + cards_compare(&cards_in_play, &game_mode)) % 4;
            println!(
                "Strongest card is the {:?} of {:?}",
                cards_in_play[win_hand_index].value, cards_in_play[win_hand_index].suit
            );

            //assume indexes 0 and 2 are of one team, as well as 1 and 3
            point_decks[win_hand_index % 2].append(&mut cards_in_play);
        } //round's over

        points_game = point_count(&point_decks, &game_mode);

        println!("\nThe round is over!");
        println!(
            "Added 10 points to team #{} for getting last trick\n",
            (win_hand_index % 2) + 1
        );

        //add last 10 to team that got last trick
        match game_mode {
            NoTrumps => points_game[win_hand_index % 2] += 20,
            _ => {
                points_game[win_hand_index % 2] += 10;
                for index in 0..2 {
                    println!(
                        "Team #{}'s points from announcments: {}",
                        index + 1,
                        points_from_announs[index]
                    );
                    points_game[index] += points_from_announs[index];
                }
            }
        }

        //check for kapo
        for index in 0..2 {
            if point_decks[index].is_empty() {
                println!(
                    "Kapo! Team #{} gets 90 points extra!",
                    ((index % 2) + 1) + 1
                );
                points_game[(index + 1) % 2] += 90;
            }
        }

        //print points from game
        println!();
        for index in 0..2 {
            println!(
                "Team #{}'s points from game: {}",
                index + 1,
                points_game[index]
            );
        }

        //vutrene check
        if points_game[(player_last_bid_index + 1) % 2] > points_game[player_last_bid_index % 2] {
            points_game[(player_last_bid_index + 1) % 2] += points_game[player_last_bid_index % 2];
            points_game[player_last_bid_index % 2] = 0;
            println!("Team #{} is inside!", (player_last_bid_index % 2) + 1);
            println!(
                "Team #{} gets all points!",
                ((player_last_bid_index + 1) % 2) + 1
            );
            println!();
            //print points from game again
            for index in 0..2 {
                println!(
                    "Team #{}'s points from game: {}",
                    index + 1,
                    points_game[index]
                );
            }
        }
        println!();

        //round points according to game mode
        for index in 0..2 {
            let round_limit = match game_mode {
                OneTrump(_) => 6,
                AllTrumps => 4,
                NoTrumps => 5,
                Pass => panic!("Pass shouldn't be possible here!"),
            };
            if points_game[index] % 10 >= round_limit {
                points_game[index] += 10;
            }
            points_total[index] += points_game[index] / 10;
            println!(
                "Team #{}'s total points: {}",
                index + 1,
                points_total[index]
            );
        } //points_game is now rounded

        //check for hanging
        //NOTE: check for hanging happens AFTER rounding the score
        if points_game[0] == points_game[1] {
            hanging_points += points_game[0];
            println!("The game is hanging!");
            println!(
                "Added points to team #{} and {} are hanging for next game!",
                (player_last_bid_index % 2) + 1,
                hanging_points
            );
        //check to add hanging points
        } else if hanging_points != 0 {
            let team_win_index = if points_game[0] > points_game[1] {
                0
            } else {
                1
            };
            points_game[team_win_index] += hanging_points;
            println!(
                "Team #{} gets the {} hanging points!",
                team_win_index, hanging_points
            );
            //reset hanging points
            hanging_points = 0;
        }

        //check for winner
        if points_total[0] >= 151 || points_total[1] >= 151 {
            //at least one team has >=151
            if points_total[0] < 151 {
                //one team is >= 151, other isn't - winner
                println!("Team #{} is the winner!", 1);
                break;
            }
            if points_total[1] < 151 {
                //one team is >= 151, other isn't - winner
                println!("Team #{} is the winner!", 0);
                break;
            }
            //both teams >=151
            if points_total[0] > points_total[1] {
                //one team has more points
                println!("Team #{} is the winner!", 0);
                break;
            }
            if points_total[1] > points_total[0] {
                //one team has more points
                println!("Team #{} is the winner!", 1);
                break;
            }
            //both team have equal points and are >=151
            println!("Both teams have >=151 but are equal! Starting another game...");
        }

        //move first player of next game with one
        init_hand_index = (init_hand_index + 1) % 4;
        println!("Enter any key to continue");
        user_input();
    }
}

fn belot_check(hand: &Vec<Card>, game_mode: &GameMode, played_card: Card, init_card: Card) -> bool {
    //returns true if there is a belot (also checks if game mode allows it)
    //if the played_card is also the first played card, then init_card should be played_card
    if played_card.value == King || played_card.value == Queen {
        let second_belot_card = if played_card.value == King {
            Queen
        } else {
            King
        };
        match game_mode {
            NoTrumps => return false,
            AllTrumps => {
                if played_card.suit == init_card.suit {
                    if hand.contains(&Card {
                        suit: played_card.suit,
                        value: second_belot_card,
                    }) {
                        println!("Belot! Added 20 points");
                        return true;
                    }
                }
            }
            OneTrump(trump_suit) => {
                if played_card.suit == *trump_suit {
                    if hand.contains(&Card {
                        suit: played_card.suit,
                        value: second_belot_card,
                    }) {
                        println!("Belot! Added 20 points");

                        return true;
                    }
                }
            }
            Pass => panic!("Pass shouldn't be possible at belot_check()"),
        }
    }
    false
}

fn bidding(init_player: usize) -> (GameMode, usize) {
    //ask players to bid, returns game mode when passes game's conditions
    //returns index of last player who bid (required for unrelated checks)

    let mut last_game_mode_index = 0;
    let mut current_player = init_player;
    let mut pass_counter = 0;
    let mut last_player_who_bid = 4;
    loop {
        println!("player #{}", current_player + 1);
        let current_bid;
        //check if bidding has occured
        if last_game_mode_index == 0 {
            println!("No one has bid yet");
            current_bid = ask_bid(7);
        } else {
            println!(
                "Current bid: {} from player #{}",
                match last_game_mode_index {
                    1 => "All Trumps",
                    2 => "No Trumps",
                    3 => "Spades",
                    4 => "Hearts",
                    5 => "Diamonds",
                    6 => "Clubs",
                    _ => panic!("at bidding() user input out of bounds!"),
                },
                last_player_who_bid + 1
            );
            current_bid = ask_bid(last_game_mode_index);
        };
        //check if not pass
        if current_bid != 0 {
            last_game_mode_index = current_bid;
            last_player_who_bid = current_player;
            pass_counter = 0;
        } else {
            pass_counter += 1;
            //check 3 passes after bid
            if last_game_mode_index != 0 && pass_counter == 3 {
                break;
            }
            //check 4 passes for no bid - restart game
            if last_game_mode_index == 0 && pass_counter == 4 {
                break;
            }
        }

        //move player to ask with one
        current_player = (current_player + 1) % 4;
    }

    (
        match last_game_mode_index {
            0 => Pass,
            1 => AllTrumps,
            2 => NoTrumps,
            3 => OneTrump(Spades),
            4 => OneTrump(Hearts),
            5 => OneTrump(Diamonds),
            6 => OneTrump(Clubs),
            _ => panic!("at bidding() user input out of bounds!"),
        },
        last_player_who_bid,
    )
}

fn ask_bid(last_game_mode_index: usize) -> usize {
    //prints possible bidding options and returns user input
    let game_mode_string = [
        "Pass",
        "AllTrumps",
        "NoTrumps",
        "Spades",
        "Hearts",
        "Diamonds",
        "Clubs",
    ];
    println!("\nSelect your bid: ");
    for index in 0..last_game_mode_index {
        println!("{}:\t{}", index + 1, game_mode_string[index]);
    }

    user_input_to_int(last_game_mode_index)
}

fn check_cards_sequence(hand: &Vec<Card>) -> (Vec<usize>, Vec<Card>) {
    //sorts hand, returns 4 ints for cards in a highest row of same suit
    //DOESN'T WORK IN 1 CASE - IN CASE OF 2 SEQUENCES IN
    //SAME SUIT, WILL REGISTER ONLY 1 (excluding cases of quinte)
    let sort_way = REGULAR_ORDER;
    let hand = sort_hand(&mut hand.clone(), sort_way);
    let cards_actual_value = cards_actual_value(&hand, sort_way);
    //final results
    let mut max_card_seqs: Vec<Card> = Vec::new();
    let mut hand_sequence_values = Vec::new();

    for spec_suit in CardSuits::iter() {
        //temp highest results
        let mut row_value: usize = 0;
        let mut temp_row_value: usize = 1;
        let mut temp_highest_card: Card = Card {
            value: Seven,
            suit: Clubs,
        };
        let mut highest_index = 1;
        for index in 0..hand.len() - 1 {
            if hand[index].suit == spec_suit && hand[index + 1].suit == spec_suit {
                //check if current and next card are in a row
                if cards_actual_value[index] == cards_actual_value[index + 1] - 1 {
                    temp_row_value += 1;
                    highest_index = index + 1;
                } else {
                    //sequence ends, record results
                    if temp_row_value > row_value {
                        row_value = temp_row_value;
                        temp_highest_card = hand[index];
                    }
                    temp_row_value = 1;
                }
            }
        }
        //do check again incase else case never occurs
        if temp_row_value > row_value {
            row_value = temp_row_value;
            temp_highest_card = hand[highest_index];
        }
        max_card_seqs.push(temp_highest_card);
        hand_sequence_values.push(row_value);
    }
    (hand_sequence_values, max_card_seqs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_card_sequence_right() {
        //hand has 1 not in a row card on right
        let hand = vec![
            Card {
                suit: Clubs,
                value: Eight,
            },
            Card {
                suit: Clubs,
                value: Nine,
            },
            Card {
                suit: Clubs,
                value: Ace,
            },
            Card {
                suit: Clubs,
                value: Ten,
            },
            Card {
                suit: Clubs,
                value: Jack,
            },
            Card {
                suit: Clubs,
                value: Queen,
            },
        ];
        assert_eq!(
            check_cards_sequence(&hand),
            (
                vec![5],
                vec![Card {
                    suit: Clubs,
                    value: Queen,
                }]
            )
        )
    }
    #[test]
    fn test_card_sequence_left() {
        //hand has 1 not in a row card on left
        let hand = vec![
            Card {
                suit: Clubs,
                value: Eight,
            },
            Card {
                suit: Clubs,
                value: Ace,
            },
            Card {
                suit: Clubs,
                value: Ten,
            },
            Card {
                suit: Clubs,
                value: Jack,
            },
            Card {
                suit: Clubs,
                value: Queen,
            },
        ];
        assert_eq!(
            check_cards_sequence(&hand),
            (
                vec![3],
                vec![Card {
                    suit: Clubs,
                    value: Queen,
                }]
            )
        )
    }
    #[test]
    fn test_card_sequence_trim() {
        //hand is only of cards in a row
        let hand = vec![
            Card {
                suit: Diamonds,
                value: Ace,
            },
            Card {
                suit: Clubs,
                value: Ace,
            },
            Card {
                suit: Diamonds,
                value: King,
            },
            Card {
                suit: Diamonds,
                value: Queen,
            },
        ];
        assert_eq!(
            check_cards_sequence(&hand),
            (
                vec![3],
                vec![Card {
                    suit: Diamonds,
                    value: Ace,
                }]
            )
        )
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
fn point_count(point_decks: &[Vec<Card>; 2], game_mode: &GameMode) -> [usize; 2] {
    //takes 2 decks and transforms cards into points for each team
    let mut points_from_decks: [usize; 2] = [0, 0];
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
            points_from_decks[index] += match points_order {
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
            points_from_decks[0] *= 2;
            points_from_decks[1] *= 2;
        }
        _ => (),
    }
    points_from_decks
}

enum PointsOrder {
    NoTrumps,
    AllTrumps,
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

fn ask_play_card(hand: &mut Vec<Card>) -> usize {
    let ans_int;
    println!("Choose a card:");
    print_hand(hand, true);
    ans_int = user_input_to_int(hand.len());
    ans_int
}

fn card_validation(
    hand: &mut Vec<Card>,
    game_mode: &GameMode,
    init_card: Card,
    cards_in_play: &Vec<Card>,
    win_hand_index: usize,
) -> Card {
    //checks if card is valid for playing
    //(needs to NOT be first played card because it is used for initial checks)
    //init_suit must be the current strongest card
    let mut has_init_suit = false;
    let mut has_higher_value = false;
    let init_card_val = TRUMP_ORDER
        .iter()
        .position(|&r| r == init_card.value)
        .unwrap();
    let card_val = cards_actual_value(hand, TRUMP_ORDER);
    for (index, card) in hand.iter().enumerate() {
        if card.suit == init_card.suit {
            has_init_suit = true;
            if card_val[index] > init_card_val {
                has_higher_value = true;
            }
        }
    }
    let mut card_to_play: Card;
    let mut card_to_play_index: usize;

    loop {
        print_cards_in_play(cards_in_play, win_hand_index);
        card_to_play_index = ask_play_card(hand);
        card_to_play = hand[card_to_play_index];
        let card_to_play_val = TRUMP_ORDER
            .iter()
            .position(|&r| r == card_to_play.value)
            .unwrap();
        //init checks, valid for every game mode
        if has_init_suit {
            if card_to_play.suit != init_card.suit {
                println!("Card's suit doesn't match the required one!");
                continue;
            }
        } else {
            //extra check for onetrump case
            match game_mode {
                OneTrump(trump_suit) => {
                    if init_card.suit != *trump_suit {
                        let mut has_trump = false;
                        for card in hand.iter() {
                            if card.suit == *trump_suit {
                                has_trump = true;
                            }
                        }
                        if has_trump {
                            //no init_suit, but has trump (this is case 2 from cards_compare())
                            if card_to_play.suit == *trump_suit {
                                println!("You don't have the required suit, but you have a trump");
                                break;
                            }
                            println!("You have a trump, but you aren't playing it!");
                            continue;
                        }
                        println!("You dont have required suit or a trump, play any");
                        break;
                    }
                }
                _ => (),
            }
            println!("You don't have the required suit, play any card");
            break;
        }

        match game_mode {
            NoTrumps => (),
            AllTrumps => {
                if has_higher_value {
                    if card_to_play_val > init_card_val {
                        break;
                    }
                    println!("You have a higher value trump, but this card isn't!");
                    continue;
                }
                println!("You don't have a higher value trump, just play a trump");
                break;
            }
            OneTrump(trump_suit) => {
                //trump case
                if init_card.suit == *trump_suit {
                    //in case of 2 cards in play - 1st card is teammate, skip this check
                    //in case of 3 cards in play - 2nd card is teammate, skip this check
                    if (cards_in_play.len() == 2 && init_card == cards_in_play[0])
                        || (cards_in_play.len() == 3 && init_card == cards_in_play[1])
                    {
                        println!("Your teammate has the highest trump, just play a trump");
                        break;
                    }
                    if has_higher_value {
                        if card_to_play_val > init_card_val {
                            break;
                        }
                        println!("You have a higher value trump, but this card isn't!");
                        continue;
                    }

                    println!("You don't have a higher value trump, just play a trump");
                    break;
                }
            }
            Pass => panic!("Pass shouldn't be possible at ask_play_card()"),
        }
        break;
    }
    hand.remove(card_to_play_index);
    card_to_play
}

fn print_cards_in_play(cards_in_play: &Vec<Card>, first_card_index: usize) {
    //prints cards_in_play depending on their count and first playing player
    for index in 0..cards_in_play.len() {
        println!(
            "\t\t\tp{}:{:?} {:?}",
            ((first_card_index + index) % 4) + 1,
            cards_in_play[index].value,
            cards_in_play[index].suit
        );
    }
    println!();
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
        let temp = hand[index];
        hand[index] = hand[temp_j];
        hand[temp_j] = temp;
        let temp_card_regular_value = card_regular_value[index];
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
        println!("player #{}:", index + 1);
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
    let mut sequence_values: [Vec<usize>; 2] = [vec![], vec![]];
    let mut max_card_seq: [Vec<Card>; 2] = [vec![], vec![]];
    //sorting hands' cards
    for index in 0..4 {
        match game_mode {
            NoTrumps => hands[index] = sort_hand(&mut hands[index], NO_TRUMP_ORDER),
            AllTrumps => hands[index] = sort_hand(&mut hands[index], TRUMP_ORDER),
            //can make it to sort trump suit in trump order
            OneTrump(trump_suit) => {
                //sort in no trump order first
                hands[index] = sort_hand(&mut hands[index], NO_TRUMP_ORDER);
                //remove trump cards
                let mut trump_cards: Vec<Card> = Vec::new();
                for card_index in 0..hands[index].len() {
                    let card = hands[index][card_index];
                    if card.suit == *trump_suit {
                        trump_cards.push(card);
                    }
                }
                //remove trump cards from hand
                hands[index].retain(|card| card.suit != *trump_suit);
                //push properly sorted trump cards
                //NOTE: trump cards are being added at end of sequence always
                //(although incidental, its actually nice)
                hands[index].extend(sort_hand(&mut trump_cards, TRUMP_ORDER));
            }
            Pass => (),
        }
        println!("player #{}:", index + 1);
        print_hand(&hands[index], false);

        //not checking for carre and cards sequences in NoTrumps
        match game_mode {
            NoTrumps => (),
            _ => {
                check_carre(&hands[index], index, points_from_announs);
                let (temp_seq_cal, temp_max_card_seq) = check_cards_sequence(&hands[index]);
                sequence_values[index % 2].extend(temp_seq_cal);
                max_card_seq[index % 2].extend(temp_max_card_seq);
            }
        }
    }
    //validating cards sequences
    let mut highest_sequences: [usize; 2] = [0, 0];
    for index in 0..2 {
        for val in sequence_values[index].iter() {
            if highest_sequences[index] < *val {
                highest_sequences[index] = *val;
            }
        }
    }
    println!("checking in a row");
    if highest_sequences[0] >= 3 && highest_sequences[1] >= 3 {
        println!("highest pass");
        if highest_sequences[0] == highest_sequences[1] {
            //highest sequences are equal
            println!("The highest sequences are of equal length");
            let max_card_actual_val = [
                cards_actual_value(&max_card_seq[0], REGULAR_ORDER),
                cards_actual_value(&max_card_seq[1], REGULAR_ORDER),
            ];
            //seq len
            //max seq - done
            //seq max card - done ,
            //[1, 0, 2, 4,      0, 0, 4, 4,     0, 1, 6, 1,   4, 1, 2, 1]
            //
            for index in 0..sequence_values[0].len() {
                if sequence_values[0][index] == highest_sequences[0] {
                    if max_card_actual_val[0][index] > max_card_actual_val[1][index] {
                        println!("Team #1 has higher seqns values");
                    } else if max_card_actual_val[0][index] < max_card_actual_val[1][index] {
                        println!("Team #2 has higher seqns values");
                    } else {
                        println!("Max cards are equal! Dropping all sequences!");
                    }
                }
            }
        } else {
            //one team has a higher sequence than other
            let team_sequence_index = if highest_sequences[0] > highest_sequences[1] {
                println!("Team #1 has longer card sequence");
                println!("Team #2's card sequences don't count");
                0
            } else {
                println!("Team #2 has longer card sequence");
                println!("Team #1's card sequences don't count");
                1
            };
        }
    }
    hands
}

fn points_from_card_sequences(sequence_values: &Vec<usize>) -> usize {
    //takes vec of sequence lenghts, turns it into points and returns
    let mut points = 0;
    for val in sequence_values {
        match val {
            3 => points += 20,
            4 => points += 50,
            5 => points += 100,
            6 => points += 100,
            7 => points += 100,
            8 => points += 120,
            _ => (),
        }
    }
    points
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
    Double,
    ReDouble,
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
