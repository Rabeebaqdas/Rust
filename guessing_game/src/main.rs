use std::io;
fn main() {
    println!("Guessing Game");
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    println!("You guessed: {}", guess);

}