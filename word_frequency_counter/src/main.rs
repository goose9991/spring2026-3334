fn most_frequent_word(text: &str) -> (String, usize) {
    let mut words: Vec<&str> = text.split_whitespace().collect();
    
    if words.is_empty() {
        return (String::new(), 0);
    }

    words.sort();

    let mut max_count = 1;
    let mut max_word: String = words[0].to_string();

    let mut current_count = 1;
    let mut current_word: String = words[0].to_string();
    
    for &word in words.iter().skip(1) {
        if word == current_word {
            current_count += 1;
        } else {
            if current_count > max_count {
                max_count = current_count;
                max_word = current_word;
            }
            current_word = word.to_string();
            current_count = 1;
        }
    }

    if current_count > max_count {
        max_count = current_count;
        max_word = current_word;
    }

    (max_word, max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count); //Most frequent word: "the" (3 times)
}