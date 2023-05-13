pub fn run() {
    // Print to console
    println!("Hello from the print.rs file.");

    // Basic formatting
    println!("{} is from {}.", "A", "B");

    // Position arguments
    println!("{0} is from {1} and {0} likes {2}.", "A", "B", "C");

    // Named arguments
    println!(
        "{name} likes to play {activity}.",
        name = "Jack",
        activity = "Rust"
    );

    // Placeholder traits
    println!("Binary: {0:b} Hex: {0:x} Octal: {0:o}", 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic match
    println!("10 + 10 = {}", 10 + 10);
}
