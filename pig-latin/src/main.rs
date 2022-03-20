/*
    Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

fn converter(string: String) -> String {
    match &string[0..1] {
        "a" | "e" | "i" | "o" | "u" | "y" => format!("{}-hay", &string),
        _ => format!("{}-{}ay", &string[1..string.len()], &string[0..1]),
    }
}

fn main() {
    assert_eq!(converter(String::from("apple")), String::from("apple-hay"));
    assert_eq!(converter(String::from("first")), String::from("irst-fay"));
}