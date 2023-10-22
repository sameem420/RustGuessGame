use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!");
    let scnum = rand::thread_rng().gen_range(1, 101); // Secret number

    loop {
        let mut guess = String::new(); // The user's guess
    

        println!("Please input your guess: ");

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Convert the user's input to a number, and handle any errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue; // Continue to the next iteration of the loop
            }
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&scnum) {
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop when the user correctly guesses the number
            }
            Ordering::Less => println!("Too low, try again"),
            Ordering::Greater => println!("Too high, try again"),
        }
    }
}

