use std::io;

fn main() {
    // Declaring variables
    let mut input = String::new();
    let temp: f64;

    // Getting user input for the temperature to check
    println!("Enter a temperature in Fahrenheit");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Converting the input to a 64 bit float
    temp = input.trim().parse().expect("Not a number");

    // Checking the input value against the freezing point in fahrenheit
if temp == 32.0 {
println!("At freezing");
} else {    
if temp > 32.0 {
        println!("Above freezing");
    } else {
        println!("Below freezing");
    } 
}
}

