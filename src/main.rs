mod password_generator;
use password_generator::*;

use std::io;

fn main() {
    let (phrase, password_length) = prompt_for_user_input();

    let password = obscure_string(&generate_password(&phrase));

    println!("Your password: {}", password);
    println!("Correct length password: {}", repeat_string_for_length(&password, &password_length));
}

fn prompt_for_user_input() -> (String, u64) {
    println!("Enter a sentence or sequence of words with capitalization and punctuation:");
    let mut phrase = String::new();
    io::stdin().read_line(&mut phrase).unwrap();

    let mut pass_len: Option<u64> = None;
    println!("How long do you want your password to be?");
    while let None = pass_len {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read the user's input
        let input = input.trim(); // Remove whitespace
        
        match input.parse::<u64>() { // Parse the String into an integer
            Ok(n) => pass_len = Some(n),
            Err(_) => println!("Please enter a positive integer") // If the string cannot be parsed
        };
    }
    (phrase, pass_len.unwrap())
}

