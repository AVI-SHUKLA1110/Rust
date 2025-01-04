use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);
    println!("The Secret number is: {}", secret_number);
    println!("Enter your guess.");
    
    let mut _guess = String::new();
    
    io::stdin()
        .read_line(&mut _guess)
        .expect("Failed to read line");

    let _guess: i32 = _guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed {}", _guess);

    match _guess.cmp(&secret_number) {
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Equal, You Win!"),
        Ordering::Less => println!("Too less!")
    }
}
