use std::{collections::HashMap, hash::Hash};

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

pub fn is_nice_v2(word: &str) -> bool {
    let mut pairs: HashMap<String, u32> = HashMap::new();
    let mut count = 0;
    let mut i = 0;
    loop {
        if word.len() - i < 2 {
            break;
        }
        let key = &word[i..i + 2].to_string();
        match pairs.get(key) {
            Some(_) => *pairs.get_mut(key).unwrap() += 1,
            None => {
                pairs.insert(key.clone(), 1);
            }
        }
        i += 2;
    }

    for i in 0..(word.len() - 2) {
        let piece: Vec<char> = word[i..i + 3].chars().collect();
        if piece.get(0).unwrap() == piece.get(2).unwrap()
            && piece.get(1).unwrap() != piece.get(0).unwrap()
        {
            count += 1;
        }
    }
    if pairs.values().filter(|&e| *e > 1).count() == 0 || count == 0 {
        return false;
    }
    true
}

pub fn nice_words(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .filter(|e| is_nice_v2(*e))
        .count() as u32
}
