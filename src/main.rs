extern crate rand;
// We add a line that lets Rust know we'll be using the rand crate as external dependency.
use std::io;
use std::cmp::Ordering;
use rand::Rng;
// The Rng trait defines methods that random number generators implement,
// and this trait must be in scope for us to use those method

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess= String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) =>num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

