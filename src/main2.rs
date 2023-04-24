use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u16 = rand::thread_rng().gen_range(1..1001);

    loop {
        println!("Please input your guess between 1 and 1000.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u16 = guess.trim().parse().unwrap();

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}