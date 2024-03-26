use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guessing Game");
   loop {
    println!("Please input your guess");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1,100);
    println!("The secret number: {}", secret_number);
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    // let guess:u32 = guess.trim().parse().expect("Please type a number");
    let guess:u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small".red()),
        Ordering::Greater => println!("{}", "Too big".red()),
        Ordering::Equal => {
            println!("{}", "You got it!".green());
            break;
        }

    }
   }


}
