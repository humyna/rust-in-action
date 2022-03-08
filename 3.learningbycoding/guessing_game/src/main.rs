use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) //The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you need to input a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp( &secret_number ) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("你赢了!");
                break;
            }
        }
    }
}
