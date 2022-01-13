mod password_generator;
use password_generator::generate_password;

use std::io;

fn main() {
    let (phrase, pass_len) = prompt_for_user_input();

    let password = generate_password(&phrase);

    println!("Your password: {}", password);
    println!("Correct length passowrd: {}", repeat_string_for_length(&password, &pass_len));
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

/// Return a string consisting of the input string repeated
/// for the specified number of chars.
fn repeat_string_for_length(s: &String, length: &u64) -> String {
    let mut out = String::new();

    let mut string_iterator = s.chars().cycle();
    for _ in 0..*length {
        out.push(string_iterator.next().unwrap());
    }

    out
}

/// Unit test for string repetition
#[test]
fn string_repeat() {
    assert_eq!(
        repeat_string_for_length(&String::from("Mark"), &22),
        String::from("MarkMarkMarkMarkMarkMa")
    );
    assert_eq!(
        repeat_string_for_length(&String::from("Mark"), &2),
        String::from("Ma")
    );
}