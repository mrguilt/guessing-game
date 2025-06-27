use std::cmp::Ordering; //Listing 2-4
use std::io;
use rand::Rng; //Listing 2-3

fn main() {
    println!("Listing 2-1 from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //Listing 2-3

    println!("The secret number is {secret_number}."); //Listing 2-3

    loop {  //Listing 2-6
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); //Listing 2-5


    //Listing 2-4
    match guess.cmp(&secret_number) { 
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => { 
            println!("You win!");  //Listing 2-7
            break;  //Listing 2-7, breaks the loop
        }
    }

    }  //Listing 2-6  
    //Adding from another bit--this is me.
   let x=5;
   let y=10;
   println!("x is {x} and y+2 is {}",y+2);
   println!("First is {} and second it {}",1,"orange");   
}