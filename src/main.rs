use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number between 1 and 100");

    let random_number: u32 = rand::rng().random_range(1..100);

    println!("please input your guess.");
    loop {

        let mut guess = String::new();

        println!("random number: {}", random_number);
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small! try again"),
            Ordering::Greater => println!("Too big! try again"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }

    }
}
