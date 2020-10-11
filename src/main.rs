fn main() {
    println!("{}", pig_latin("first apple"));
}

fn pig_latin(s: &str) -> String {
    let mut new_string = String::new();

    for word in s.split_whitespace() {
        let c = word.chars().nth(0).unwrap().to_lowercase().to_string();

        let mut new_word = String::new();
        if  c == "a" || c == "e" || c == "i" || c == "o" || c == "u" || c =="y" {
            new_word = format!("{}-hay ", word);
        } else {
            new_word = format!("{}-{}ay ", &word[1..], c);
        }

        new_string.push_str(&new_word);
    }

    new_string
}