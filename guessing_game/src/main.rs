use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(0..=1000);

    loop {
        println!("Please input your guess...");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Has to be a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
