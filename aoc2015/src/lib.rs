pub fn not_quite_list(path:&str) -> i32 {
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut instructions = vec![0,0];
    for token in file_content.chars() {
        match token {
            '(' => instructions[0] += 1,
            ')' => instructions[1] += 1,
            _ => (),
        }
    }
    instructions[0] - instructions[1]
}