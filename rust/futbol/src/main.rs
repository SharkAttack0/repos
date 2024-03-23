use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Player {
    name: &'static str,
    score: u32,
}
impl Player {
    fn new(name: &'static str, score: u32) -> Self {
        Player { name, score }
    }
}

//2 otbora, ednakva stoinost, rng
fn main() {
    let mut igrachi = vec![];
    igrachi.push(Player::new("Tedo", 10));
    igrachi.push(Player::new("Asenov", 10));
    igrachi.push(Player::new("Monka", 9));
    igrachi.push(Player::new("Kamenov", 8));
    igrachi.push(Player::new("Vlahov", 8));
    igrachi.push(Player::new("Gena", 8));
    igrachi.push(Player::new("Ice", 7));
    igrachi.push(Player::new("Dimitrov", 6));
    igrachi.push(Player::new("Vase", 6));
    igrachi.push(Player::new("Zarev", 5));
    igrachi.push(Player::new("Bobka", 5));
    let mut rng = thread_rng();
    igrachi.shuffle(&mut rng);

    let sum = igrachi[0..5]
        .into_iter()
        .reduce(|acc, player|  {
            name: "",
            score: acc.score + player.score,
        }
        .unwrap();

    println!("{sum:?}");
}
