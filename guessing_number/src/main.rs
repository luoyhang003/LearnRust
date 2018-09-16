extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_num: u32 = rand::thread_rng().gen_range(1, 101);
    println!("Here is th secret number: {}", secret_num);
    loop {
        println!("Please input the number you guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong Input!");
                continue;
            }
        };
        println!("You guessed {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Win!");
                break;
            }
        }
    }
}
