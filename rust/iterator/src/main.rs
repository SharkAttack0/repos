fn main() {
    let mut shoe_store: Vec<Shoe> = Vec::new();
    shoe_store.push(Shoe {name: String::from("sandal"), size: 12});
    shoe_store.push(Shoe {name: String::from("boot"), size: 13});
    shoe_store.push(Shoe {name: String::from("sneaker"), size: 12});
    
    let my_size: u32 = 12;
    let shoes_for_me = specific_shoe_size(shoe_store, my_size);

    println!("{:?}", shoe_store);

}

#[derive(Debug)]
struct Shoe {
    name: String,
    size: u32,
}

fn specific_shoe_size(shoe_store: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoe_store.into_iter().filter(|s| s.size == shoe_size).collect()
}
