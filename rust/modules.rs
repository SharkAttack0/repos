mod front {
    pub mod front_of_front {
        pub fn returnFive() -> u32 {
            println!("called returnFive()");
            5   
        }
    }
}

mod back {
    use front::front_of_front::returnFive;
    pub fn callReturnFive() {
        println!("called callReturnFive()");
        let five = returnFive();
    }
}

fn main() {
    back::callReturnFive();
}
