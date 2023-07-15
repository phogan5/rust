use std::io; //Import io from the standard library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    
    loop {
        println!("Please input your guess.");

    let mut guess = String::new();
        //Use let to assign variable
        //mut indicates a variable that can change (by default, variables are immutable)
        //String::new() calls a new instance of type String and assigns it to guess

    io::stdin() //Use the stdin function from the io library
        .read_line(&mut guess) //calls read_line on stdin(), passing &mut guess as an argument to read_line to set the variable = guess
        .expect("Failed to read line"); //Handles any 'err' Results that may come from read_line and passes a message to stdout (error handling)
    //Can also be written as io::stdin().read_line(&mut guess).expect("Failed to read line");
    //But the above is easier to read

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    } //Match statement: The program uses cmp (compare crate) to compare guess with secret_number. It will go through each line until it matches a part of the statement
    }
}