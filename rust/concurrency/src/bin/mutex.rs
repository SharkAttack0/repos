use std::sync::Mutex;

fn main() {
    println!("mutex file");
    let m = Mutex::new(1);
    {
        let mut num = m.lock().unwrap();
        println!("{num}");
        *num = 6;
    }
    println!("m = {:?}", m);
}
