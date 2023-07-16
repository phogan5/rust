fn main() {
    a_normal_function(5);

    multiple_parameters(5, 'm');

    let x = return_values1();
    println!("The value of x is {x}");
    //The value of x is 5
    
    let y = return_values2(10);
    println!("The value of y is {y}")
    //The value of y is 11
}

fn a_normal_function(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_parameters(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn return_values1() -> i32 {
    5
}

fn return_values2(x: i32) -> i32 {
    x + 1
}



