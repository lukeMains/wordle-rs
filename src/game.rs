pub fn reduce_answers(guess: String, info: String, word_list: Vec<&str>) -> Vec<&str> {
    let mut word_list = word_list.clone();
    
    // Get letter data:
    let black_letters = guess
        .chars()
        .enumerate()
        .filter(|(i, _)| {info.chars().nth(*i).unwrap() == 'b'});  //TODO: Ignore the index part of the tuple
    let yello_letters = guess
        .chars()
        .enumerate()
        .filter(|(i,_)| {info.chars().nth(*i).unwrap() == 'y'});
    let green_letters = guess
        .chars()
        .enumerate()
        .filter(|(i,_)| {info.chars().nth(*i).unwrap() == 'g'});


    // Filter possible answers:
    for (index, letter) in green_letters {
        word_list = word_list
            .into_iter()
            .filter(|word| {
                word.chars().nth(index).unwrap() == letter
            })
            .collect();
    }
    for (index, letter) in yello_letters {
        word_list = word_list
            .into_iter()
            .filter(|word| {
                word.contains(letter) && 
                word.chars().nth(index).unwrap() != letter
            })
            .collect();
    }
    for (_, letter) in black_letters {
        word_list = word_list
            .into_iter()
            .filter(|word| {
                !word.contains(letter)
            })
            .collect();
    }

    return word_list;
}