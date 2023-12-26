// Source code store in UTF-8 (unicode) format
// .chars() method returns an iterator over the characters in a string
// String is a wrapper over a Vec<u8>
// &str is a slice of a string slice (&[u8])

fn main() {
    let sentence = String::from("Rust is a systems programming language that runs blazingly fast");
    let new_sentence = sentence.replace("fast", "slow");
    println!("{} -> {}", sentence, new_sentence);


    println!("words in reverse");
    for word in new_sentence.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = new_sentence.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("{}", string);


    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used     charracter: {}", string);
    println!("Trimmed  charracter: {}", trimmed_str);
}