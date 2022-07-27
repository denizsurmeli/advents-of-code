pub fn is_nice(word: &str) -> bool {
    let forbidden = vec!["ab", "cd", "pq", "xy"];
    let vowels = "aeiou".to_string();
    let mut count = 0;
    for term in forbidden {
        if word.contains(term) {
            return false;
        }
    }
    for char in word.chars() {
        if vowels.contains(char) {
            count += 1;
        }
    }
    if count < 3 {
        return false;
    }

    count = 1;
    let mut last_seen = word.chars().nth(0).unwrap();
    for c in word.chars().skip(1) {
        if c == last_seen {
            count += 1;
            if count == 2 {
                break;
            }
        } else {
            last_seen = c;
            count = 1;
        }
    }
    if count < 2 {
        return false;
    }
    true
}

pub fn nice_words(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .filter(|e| is_nice(*e))
        .count() as u32
}
