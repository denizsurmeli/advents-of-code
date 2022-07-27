// build x-y axis, thing starts from zero
// hashmap to save visited locations and increment once

use std::collections::HashMap;

#[derive(Debug,PartialEq)]
enum Directions{
    Up,
    Down,
    Left,
    Right,
}

pub fn house_visits(path:&str) -> u32{
    let mut history:HashMap<(i32,i32),u32> = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    let file_content = std::fs::read_to_string(path).unwrap();
    for e in file_content.chars() {
        match history.get(&(x,y)) {
            Some(_) => *history.get_mut(&(x,y)).unwrap() += 1,
            None => {history.insert((x,y), 1);},
        } 
        match e {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => (),
        } 
    }

    history.iter().count() as u32

}