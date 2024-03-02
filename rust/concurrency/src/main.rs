use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    threads_move();
    threads_join();
    two_threads_mpsc();
}

fn threads_move() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let num1 = 5;
    let num2 = 6;
    let a = thread::spawn(move || {
        println!("{:?}", v1);
        println!("{num1}");
    });
    for i in 1..=3 {
        println!("{i}");
        thread::sleep(Duration::from_millis(500));
    }
    a.join().unwrap();
    println!("{:?}", v2);
    println!("{num2}");
}
fn threads_join() {
    let first = thread::spawn(|| {
        for i in 1..=5 {
            println!("1 {i}");
            thread::sleep(Duration::from_millis(500));
        }
        println!("1 done");
    });

    let second = thread::spawn(|| {
        for i in 1..=10 {
            println!("2 {i}");
            thread::sleep(Duration::from_millis(500));
        }
        println!("2 done");
    });

    first.join().unwrap();
    second.join().unwrap();

    for i in 1..=5 {
        println!("main {i}");
        thread::sleep(Duration::from_millis(500));
    }
    println!("main done");
}

fn two_threads_mpsc() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("1"),
            String::from("1"),
            String::from("1"),
        ];

        for val in vals {
            tx.send(val);
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2"),
            String::from("2"),
            String::from("2"),
            String::from("2"),
            String::from("2"),
            String::from("2"),
            String::from("2"),
        ];

        for val in vals {
            tx1.send(val);
            thread::sleep(Duration::from_millis(500));
        }
    });
    println!("Started recieving");

    for recv in rx {
        println!("{recv}");
        thread::sleep(Duration::from_millis(500));
    }

    println!("Done recieving");
}
