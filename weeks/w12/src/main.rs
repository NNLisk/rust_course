use std::io;

fn main() {
    let x = "Rust";
    let y = "No";

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().to_lowercase().as_str() {
        "rust" => println!("So you appreciate Rust? That's great! Thank you!"),
        "no" => println!("So you like nothing? Alright then... :)"),
        _ => println!("It seems that you like {}.", input.trim())
    };

    
}
