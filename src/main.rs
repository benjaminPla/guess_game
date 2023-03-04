use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

fn main() {
    println!("{}", "Guess the number!".blue().bold());

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guesses: u32 = 0;

    println!("{}", "Please input your guess:".blue());
    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                guesses += 1;
                num
            },
            Err(_) => {
                guesses += 1;
                println!("{}", "Please enter a valid number".red());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small, try again!".red()),
            Ordering::Greater => println!("{}", "Too big, try again!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                println!("{}", ["Total guesses:", &guesses.to_string()].join(" ").yellow());
                break;
            }
        }
    }
}
