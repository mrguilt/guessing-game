use std::io;
use rand::Rng; //Listing 2-3

fn main() {
    println!("Listing 2-1 from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //Listing 2-3

    println!("The secrent number is {secret_number}."); //Listing 2-3
    
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    //Adding from another bit--this is me.
    let x=5;
    let y=10;
    println!("x is {x} and y+2 is {}",y+2);
    println!("First is {} and second it {}",1,"orange");   
}