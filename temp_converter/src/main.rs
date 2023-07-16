use std::io;
fn main() {
    loop {
        println!("Enter [C] for Celsius -> Fehrenheit \nEnter [F] for Fehrenheit -> Celsius");

        let mut input_choice = String::new();
        io::stdin()
            .read_line(&mut input_choice)
            .expect("Failed to read line");

        println!("You entered: {input_choice}");

        let choice = input_choice
            .trim()
            .chars()
            .next();


        if choice == Some('C') {
            println!("Converting Celsius to Fehrenheit...");
                convert_to_f()
        } else if choice == Some('F') {
            println!("Converting Fehrenheit to Celsius...");
                convert_to_c()
        }
        else {
            println!("ERROR: Enter a [C] or [F]");
            continue
        }
    }
}

fn convert_to_f () {
    println!("Enter a value in Celsius to be converted to Fehrenheit...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let inputint: i32 = input
        .trim()
        .parse()
        .expect("Invalid input");
    println!("You entered: {inputint}");
}

fn convert_to_c () {
    println!("Enter a value in Fehrenheit to be converted to Celsius...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let inputint: i32 = input
        .trim()
        .parse()
        .expect("Invalid input");
    println!("You entered: {inputint}");
}
