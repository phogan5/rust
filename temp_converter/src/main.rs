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
    let inputint: f64 = input
        .trim()
        .parse()
        .expect("Invalid input");
    println!("You entered: {inputint}");

    //Begin math
    let result = (inputint * 1.8 + 32.0) as f64;
    println!("{} degrees C is equal to {} degrees F", inputint, result);

}

fn convert_to_c () {
    println!("Enter a value in Fehrenheit to be converted to Celsius...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let inputint: f64 = input
        .trim()
        .parse()
        .expect("Invalid input");
    println!("You entered: {inputint}");

    //Begin math
    let result = ((inputint - 32.0) * (5.0/9.0)) as f64;
    println!("{input} degrees F is equal to {result} degrees C");
}
