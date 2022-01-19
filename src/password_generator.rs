/// Create a password from the first letter and all punctuation of each 
/// word in a phrase or sentence.
pub fn generate_password(phrase: &String) -> String {
    let mut password = String::new();

    for word in phrase.split(" ") { // Iterate over each substring, delimited with " "
        let word = word.trim(); // Trim off whitespace and newlines

        // If the entire substring is a number, add all of it to the password
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

/// "Obscure" a string by replacing some characters (like 't') with other
/// simmilar characters (like '+')
pub fn obscure_string(s: &String) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 1 { // Operate on every second character
            out.push(match c {
                't' => '+',
                'o' => '0',
                'i' => '1',
                'I' => '1',
                'B' => '&',
                'l' => '|',
                'A' => '^',
                'k' => '<',
                _ => c
            });
        } else { out.push(c) }
    }

    out
}

/// Return a string consisting of the input string repeated
/// for the specified number of chars.
pub fn repeat_string_for_length(s: &String, length: &u64) -> String {
    let mut out = String::new();

    let mut string_iterator = s.chars().cycle();
    for _ in 0..*length {
        out.push(string_iterator.next().unwrap());
    }

    out
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

/// Unit test for string obstruction
#[test]
fn string_obstruction() {
    assert_eq!(
        obscure_string(&String::from("tttoooiiiIIIBBBlllAAA")),
        "tt+oo0ii1II1BB&ll|AA^"
    );
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