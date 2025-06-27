use std::io;

fn main() {
    println!("Listing 2-1 from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html");
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    //Adding from another bit--this is me.
    let x=5;
    let y=10;
    println!("x is {x} and y+2 is {}",y+2)   
}