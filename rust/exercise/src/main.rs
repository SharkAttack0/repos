fn main() {
    let player = Box::new(PlayerData{health:100,
        mana: 50});
    print!("{:p}", player);
    let player2 = Box::new(PlayerData{ health:100,
    mana: 50});

}
#[derive(Debug)]
struct PlayerData {
    health: u32,
    mana: u32,
}

