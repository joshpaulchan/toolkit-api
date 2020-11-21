use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let target = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    match target.cmp(&guess) {
        Ordering::Less => println!("guess was too high!"),
        Ordering::Greater => println!("guess was too low!"),
        Ordering::Equal => println!("right on target!"),
    }

    println!("You wrote: {}, target was {}", guess, target);
}
