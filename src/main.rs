use std::io;
use std::f64::consts::PI;

fn main() {
    let mut input = String::new();

    // Ask for radius
    println!("Enter radius:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let radius: f64 = input.trim().parse().expect("Please enter a number");

    input.clear();

    // Ask for chord
    println!("Enter chord:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let chord: f64 = input.trim().parse().expect("Please enter a number");

    // Calculate alpha (radians)
    let alpha = 2.0 * (chord / (2.0 * radius)).asin().to_degrees(); // Convert to degrees

    // Arc length
    let length = radius * alpha * PI / 180.0;

    println!("Arc length: {}", length);

    // Wait before closing
    println!("Press Enter to exit...");
    let mut exit = String::new();
    io::stdin().read_line(&mut exit).unwrap();
}