// Wordle Game
mod helpers;
mod game;

// Imports
use std::fs;

fn main() {
    /* Parse Answers File */
    let path = "C:\\Users\\luke_\\Desktop\\Wordle\\answers.txt";
    let contents = String::from(
        fs::read_to_string(path)
            .expect("Error: Couldn't read file!")
        );

    let temp = contents.replace("\"", "");
    
    let start = 1;
    let end = temp.len() - 1;
    let answers: Vec<&str> = temp[start..end]
        .split(',')
        .collect();

    let mut possible_answers = answers;

    loop {
        // Get user guess:
        let guess = helpers::prompt("Guess");
        if "quit" == guess {break;};
        let info = helpers::prompt("Color");
        

        // Reduce answer pool
        possible_answers = game::reduce_answers(guess, info, possible_answers);
        println!("Answers: {}", possible_answers.len());

        //Get letter counts
        let temp_list = possible_answers.clone();
        let counts = helpers::get_letter_count(temp_list);
        println!("Counts: {}", counts.len());
        for (l, c) in counts.most_common_ordered() {
            println!("{}: {}", l, c);
        }

        if possible_answers.len() <= 15 {
            println!("Possible Answers:");
            for word in &possible_answers {   // This needs to be borrowed as a reference? because the for -> iterator will consume the vector.
                println!("  {}", word);
            }
        }
    }
}