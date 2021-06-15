use std::io;

fn main() {
    println!("Please enter a temperature in fahr:");
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read a line of user input.");
    let fahr: f32 = input_string
        .trim()
        .parse()
        .expect("Input string was not a float.");
    let celsius = 5.0 * (fahr - 32.0)/ 9.0;
    println!("{} °F = {} °C", fahr, celsius);
}
