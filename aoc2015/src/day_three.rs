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

pub fn update_state(h:&mut HashMap<(i32,i32), u32>,
x:&mut i32,
y:&mut i32,
e:char ) {
    match h.get(&(*x,*y)) {
        Some(_) => *h.get_mut(&(*x,*y)).unwrap() += 1,
        None => {h.insert((*x,*y), 1);},
    } 
    match e {
        '^' => *y += 1,
        'v' => *y -= 1,
        '<' => *x -= 1,
        '>' => *x += 1,
        _ => (),
    } 
}

pub fn house_visits_with_robot(path:&str) -> u32{
    let mut history:HashMap<(i32,i32),u32> = HashMap::new();
    let mut x_robot = 0;
    let mut y_robot = 0;
    let mut x_santa:i32 = 0;
    let mut y_santa = 0;

    let file_content = std::fs::read_to_string(path).unwrap();
    for (i,e) in file_content.chars().enumerate() {
        if i % 2 == 0 {
            update_state(&mut history, &mut x_santa, &mut y_santa, e);
        }
        else {
            update_state(&mut history,&mut x_robot, &mut y_robot,e);
        }

    }

    history.iter().count() as u32
}