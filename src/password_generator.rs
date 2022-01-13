/// Create a password from the first letter and all punctuation of each 
/// word in a phrase or sentence.
pub fn generate_password(phrase: &String) -> String {
    let mut password = String::new();

    for word in phrase.split(" ") { // Iterate over each substring, delimited with " "
        let word = word.trim(); // Trim off whitespace and newlines

        // If the substring is a number, add all of it to the password
        if let Ok(_) = word.parse::<f64>() {
            password.push_str(word);
            continue; // Move the next word
        }

        // If the first letter of the substring exists, append it to `password`
        if let Some(s) = word.get(..1) {
            password.push_str(s);
        }
        
        // Add any punctuation characters in the input to the password
        for c in word.chars() {
            if c.is_ascii_punctuation() {
                password.push(c)
            }
        }
    }
    password // Return the password
}

/// Unit test for password generation
#[test]
fn password_gen() {
    let phrase1 = String::from(
        "Mr. Moynihan teaches Computer Science at Servite High School in room 111"
    );
    let phrase2 = String::from("Mark took Lisa to Disneyland on March 15");

    assert_eq!(
        generate_password(&phrase1),
        "M.MtCSaSHSir111"
    );
    assert_eq!(
        generate_password(&phrase2),
        "MtLtDoM15"
    );
}