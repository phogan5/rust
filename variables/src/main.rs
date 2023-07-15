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
}

