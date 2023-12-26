
fn naive_capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

fn ascii_capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_ascii_uppercase().to_string() + c.as_str(),
    }
}


fn main() {
    let names: Vec<String> = vec![
        "john".to_string(),
        "jane".to_string(),
        "春巻き".to_string(),
        "œosasss".to_string(),
        "žalazaar".to_string(),
        "øöööö".to_string(),
        "ænnout".to_string(),
    ];
    
    for name in names.iter() {
        println!("Hello (naive_capitalize), {} => {}", name, naive_capitalize(name));
        println!("Hello (ascii_capitalize), {} => {}", name, ascii_capitalize(name));
        println!("")
    }
}