use std::io;
//https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// to be continued

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) //The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
