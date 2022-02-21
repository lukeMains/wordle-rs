// Imports
use::std::io::{self, Write};
use counter::Counter;

pub fn prompt(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    
    io::stdin()
        .read_line(&mut input)
        .expect("Error: Couldn't read input!");

    return input.trim().to_string();
}

pub fn get_letter_count(word_list: Vec<&str>) -> Counter<char>  {
    /* Takes an array of strings and returns a counter object  */
    let mut counts = "".chars().collect::<Counter<_>>();
    let word_list_copy = word_list.clone();
    for word in &word_list_copy {
        counts += word
        .chars()
        .collect::<Counter<_>>();
    }
    return counts;
}

pub fn _get_digraph_count() {
    todo!();
}