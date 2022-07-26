pub fn not_quite_list(path:&str) -> i32 {
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut instructions = vec![0,0];
    for token in file_content.chars(){
        match token {
            '(' => instructions[0] += 1,
            ')' => instructions[1] += 1,
            _ => (),
        }
    }
    instructions[0] - instructions[1]
}

pub fn not_quite_list_p2(path:&str) -> i32 {
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut instructions = vec![0,0];
    for (i,token) in file_content.chars().enumerate() {
        match token {
            '(' => instructions[0] += 1,
            ')' => instructions[1] += 1,
            _ => (),
        }
        if instructions[0] - instructions[1] == -1 {
            return i as i32 + 1;
        }
    }
    file_content.len() as i32
}