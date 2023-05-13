// Variables hold primitive data or references for data
// Variables are immutable by default
// Rust is a block-scoped language
pub fn run() {
    let name = "Jack";
    let mut age = 24;
    println!("My name is {} and I am {}.", name, age);
    age = 25;
    println!("My name is {} and I am {}.", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Jack", 24);
    println!("{} is {}..", my_name, my_age);
}
