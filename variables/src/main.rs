fn main() {
    //Mutability
    let mut  x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is {x}");

    //Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS} is a constant, it can NEVER change.");

    //Shadowing
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("Inside the scope, the value of z is {z}");
    }

    println!("Outside the scope, the value of z is {z}");

    //Data types and math
        //addition
        let sum = 5 + 10;

        //subtraction
        let difference = 95.3 - 4.5;

        //multiplication
        let product = 4 * 30;

        //division
        let quotient = 56.7 / 32.2;
        let truncated = -5/3; //results in -1

        //remainder
        let remainder = 43 % 5;
        
        println!("Sum: {sum}...Difference: {difference}...Product: {product}...Quotient: {quotient}...Truncated: {truncated}...remainder: {remainder}");

    //Compound data types:
    //Tuples
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is {y}");
    //Retrieving values by destructuring
    
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let six_point_four = x.1;
    let _one = x.2;
    println!("The value of six_point_four is {six_point_four}");
    //Retrieving values using dot notation
    

    //Arrays
    let _a = [1, 2, 3, 4, 5];
    //Arrays are comma seperated, in square brackets
    
	let a: [i32; 5] = [1, 2, 3, 4, 5];
	//Specifying an arrays type

	let first = a[0]; //1
	let second = a[1]; //2
    println!("First: {first}...Second: {second}");
	//Accessing elements of an array
}

