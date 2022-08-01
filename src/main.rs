use std::io;
fn main() {
    println!("Weolcome to - GUESS THE NUMBER -");
    println!("Please, guess a number between 1 and 100");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
