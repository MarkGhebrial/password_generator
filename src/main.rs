use std::io;

fn main() {
    println!("Enter a sentence or sequence of words with capitalization and punctuation:");
    let mut phrase = String::new();
    io::stdin().read_line(&mut phrase).unwrap();

    // Ask the user for the length of their desired password
    let pass_len = prompt_password_length();

    let mut password = String::new();

    for word in phrase.split(" ") { // Iterate iver each substring, delimited with " "
        let word = word.trim(); // Trim off whitespace and newlines

        // If the substring is a number, add all of it to the password
        if let Ok(_) = word.parse::<f64>() {
            password.push_str(word);
            continue;
        }

        // If the first letter of the substring exists, append it to `password`
        if let Some(s) = word.get(..1) {
            password.push_str(s);
        }
        
        // Add all punctuation to the password
        for c in word.chars() {
            if c.is_ascii_punctuation() {
                password.push(c)
            }
        }
    }

    println!("Your password:  {}", password);
    println!("Correct length passowrd: {}", repeat_string_for_length(&password, &pass_len));
}

fn prompt_password_length() -> u64 {
    let mut pass_len: Option<u64> = None;
    println!("How long do you want your password to be?");
    while let None = pass_len {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read the user's input
        let input = input.trim(); // Remove whitespace
        match input.parse::<u64>() {
            Ok(n) => pass_len = Some(n),
            Err(_) => println!("Please enter a positive integer")
        };
    }
    pass_len.unwrap()
}

fn repeat_string_for_length(s: &String, length: &u64) -> String {
    let mut out = String::new();

    let mut string_iterator = s.chars().cycle();
    for _ in 0..*length {
        out.push(string_iterator.next().unwrap());
    }

    out
}

#[test]
fn string_repeat() {
    assert_eq!(
        repeat_string_for_length(
            &String::from("Mark"),
            &22
        ),
        String::from("MarkMarkMarkMarkMarkMa")
    );
}