
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng; // not rng, but Rng

fn main() {
    println!("Game");

    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("cannot read.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("input is {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("correct");
                break;
            }
        }
    }
}

