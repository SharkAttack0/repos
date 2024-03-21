#![allow(unused)]

use std::default;
use std::fmt::write;
use std::io;
use std::iter;
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

//can create and shuffle deck
//can print all or one hand
//can add cards to hands
//can compare cards based on the game mode
//can check for cards in a row
//can check for carre (4 cards of equal value)
//can evaluate point system from cards
//can take user input
//can print the table (cards in play)
//can add points from announcments
//can play a card on table (only when allowed)
//can add 10 points to team getting last trick
//can round points at end of game
//can get bidding from players (without contra)

//TO BE DONE:
//change init player:
//bidding's contra
//belot check
//compare tierces/quarters/quintes and remove the weaker ones (do so for carre if necessary)
//
//at first look, bots seem to be quite easy to add (and to make them good, too)
//when game is compeletely finished, add bots
//idea for bots: put card_validation() in a for each card in hand cycle
//get vec of legal cards to be played and create condition to decide
//which one to play (or simply randomize it)
//if bot is first, get dominant cards and play them first
//add awareness of teammate's changed bid?
//(example: teammate called clubs and game mode is alltrumps, when no dominant card
//is available, play clubs)
//quite easy to have a couple of difficulties, too (i think)
//possible big additions:
//gui
//multiplayer
//bots (whether to play with them solo or replace a missing player)
//shiii this could be an actual complete game
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
    let mut points_total: [usize; 2] = [0, 0];
    let mut points_game: [usize; 2];
    let mut init_player = 0;
    //init values are not set at end
    let mut win_hand_index: usize = 0;
    let mut init_hand_index: usize = 0;
    loop {
        points_game = [0, 0];
        let (mut hands, mut deck) = new_game();
        let game_mode: GameMode = bidding(init_player);
        match game_mode {
            Pass => {
                println!("\nAll players passed! Restarting...\n");
                continue;
            }
            _ => println!("\nThe game mode is {:?}\n", game_mode),
        }
        let mut points_from_announs: [usize; 2] = [0, 0];
        
        hands = continue_game(hands, &mut deck, &game_mode, &mut points_from_announs);

        let mut cards_in_play: Vec<Card> = Vec::with_capacity(4);
        let mut point_decks: [Vec<Card>; 2] = [vec![], vec![]];

        //actual playing
        for card_index in 0..hands[0].len() {
            //init_card value here should never be of actual use
            let mut init_card = Card {
                value: Seven,
                suit: Clubs,
            };
            //for for current turn
            for hand_index in 0..4 { 
                print_cards_in_play(&cards_in_play, win_hand_index);
                println!("player #{}", ((hand_index + win_hand_index) % 4) + 1);
                //skip checks for first card
                //first player on each turn *almost* fixed
                if hand_index == 0 {
                    let init_card_index = ask_play_card(&mut hands[win_hand_index]);
                    init_card = hands[win_hand_index][init_card_index];
                    hands[win_hand_index].remove(init_card_index);
                    cards_in_play.push(init_card);
                } else {
                    init_card = cards_in_play[cards_compare(&cards_in_play, &game_mode)];
                    cards_in_play.push(card_validation(
                        &mut hands[(hand_index + win_hand_index) % 4],
                        &game_mode,
                        init_card,
                        &cards_in_play,
                    ));
                }
            }
            print_cards_in_play(&cards_in_play, win_hand_index);
            //win_hand_index is the same index as first player of next turn
            win_hand_index = cards_compare(&cards_in_play, &game_mode);
            println!(
                "Strongest card is the {:?} of {:?}",
                cards_in_play[win_hand_index].value, cards_in_play[win_hand_index].suit
            );

            //assume indexes 0 and 2 are of one team, as well as 1 and 3
            point_decks[win_hand_index % 2].append(&mut cards_in_play);
        }

        //round's over
        println!("\nThe round is over!");
        println!(
            "Added 10 points to team #{} for getting last trick\n",
            (win_hand_index % 2) + 1
        );

        points_game = point_count(point_decks, &game_mode);

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
                    //maybe make it so at NoTrumps it doesn't check for announs
                    points_game[index] += points_from_announs[index];
                }
            }
        }

        println!();
        for index in 0..2 {
            println!(
                "Team #{}'s points from game: {}",
                index + 1,
                points_game[index]
            );
        }
        println!();
        for index in 0..2 {
            //round points according to game mode
            let round_limit = match game_mode {
                OneTrump(_) => 6,
                AllTrumps => 4,
                NoTrumps => 5,
                Pass => panic!("Pass shouldn't be possible here!"),
            };
            if points_game[index] % 10 >= round_limit {
                points_game[index] += 10;
            }
            points_total[index] += (points_game[index] / 10);
            println!(
                "Team #{}'s total points: {}",
                index + 1,
                points_total[index]
            );
        }

        for index in 0..2 {
            if points_total[index] >= 151 {
                if points_total[(index + 1) % 2] < 151 {
                    //one team is >= 151, other isn't - winner
                    println!("Team #{} is the winner!", index + 1);
                    break;
                }
                //both teams >=151
                if points_total[index] > points_total[(index + 1) % 2] {
                    //one team has more points
                    println!("Team #{} is the winner!", index + 1);
                    break;
                }
                //both team have equal points
                println!("Both teams have 151 but are equal! Starting another game...");
            }
        }
        //move first player with one
        init_hand_index += 1;
        if init_hand_index == 4 {
            init_hand_index = 0;
        }
        println!("Enter any key to continue");
        user_input();
    }
}

fn bidding(init_player: usize) -> GameMode {
    let mut last_game_mode_index = 0;
    let mut current_player = init_player;
    let mut pass_counter = 0;
    let mut last_player_who_bid = 4;
    loop {
        println!("player #{}", current_player + 1);
        let mut current_bid;
        if last_game_mode_index == 0 {
            println!("No one has bid yet");
            current_bid = ask_bid(current_player, 7);
        } else {
            println!(
                "Current bid: {} from player #{}",
                match last_game_mode_index {
                    1 => "AllTrumps",
                    2 => "NoTrumps",
                    3 => "OneTrump(Spades)",
                    4 => "OneTrump(Hearts)",
                    5 => "OneTrump(Diamonds)",
                    6 => "OneTrump(Clubs)",
                    _ => panic!("at bidding() user input out of bounds!"),
                },
                last_player_who_bid + 1
            );
            current_bid = ask_bid(current_player, last_game_mode_index);
        };
        //check if pass
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

        current_player += 1;
        if current_player > 3 {
            current_player = 0;
        }
    }

    match last_game_mode_index {
        0 => Pass,
        1 => AllTrumps,
        2 => NoTrumps,
        3 => OneTrump(Spades),
        4 => OneTrump(Hearts),
        5 => OneTrump(Diamonds),
        6 => OneTrump(Clubs),
        _ => panic!("at bidding() user input out of bounds!"),
    }
}

fn ask_bid(player: usize, last_game_mode_index: usize) -> usize {
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
                println!("\tWOW! 8 in a row! This hand has a quinte AND a tierce!!!\n");
                points_count[hand_index % 2] += 120;
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
    //      from announcments for each team and adds it to the points_from_decks at the end
    //  2:  Add or remove points based on capot, vutrene and others
    //      both additions could be in their own function
    //
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
//bidding - init player, calls,
//next player - repeat
//if raising bid is possible, ask each player again
//on 3 passes in a row, start game, on 4 passes - restart

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
    let mut ans_int;
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
        print_cards_in_play(cards_in_play, 0);
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

        //init.suit == true
        //card_to_play.suit == init_card.suit

        //no init.suit - any (needs extra check for trump)
        //init.suit - has it:
        //init.suit == trump: check for > value
        //
        //OneTrump - no init.suit BUT have trump - play trump
        match game_mode {
            NoTrumps => (),
            AllTrumps => {
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

fn print_cards_in_play(cards_in_play: &Vec<Card>, mut first_card_index: usize) {
    //prints cards_in_play depending on their count and first playing player
    for index in 0..cards_in_play.len() {
        let i = (index + first_card_index) % cards_in_play.len();
        println!(
            "\t\t\tp{}:{:?} {:?}\n",
            i + 1,
            cards_in_play[i].value,
            cards_in_play[i].suit
        );
    }
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
