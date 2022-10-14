use std::io;
use std::cmp::Ordering;
use rand::Rng;
// this section is called the prelude
// std is the standard library. io is for input output

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input your guess");
        // let creates the variable
        // let apples = 5 without the mut key makes the variable immutable. Rust is immutable by default
        // make guess a new instance of string.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if parse is successful, return value will go to num and num will go to guess
            Err(_) => continue, // _ is a catch all value. Continue tells program to go to next iteration of loop
        };

        println!("You guessed: {guess}");

        // let guess : u32 = guess.trim().parse().expect("Please type a number");
        // cmp returns Ordering which is an enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                print!("You win!");
                break;
            },
        }
    }


}
