// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age = 18;
    let checked_id = false;

    // if/else
    if checked_id {
        if age >= 21 {
            println!("What would like to drink?");
        } else {
            println!("Sorry, you have to leave.");
        }
    } else {
        println!("I'll need to check your ID.");
    }

    // Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
