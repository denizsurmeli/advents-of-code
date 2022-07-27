use std::hash;

use aoc2015::day_four::find_hash;
use aoc2015::day_one::not_quite_list_p2;
use aoc2015::day_three::house_visits_with_robot;
use aoc2015::day_two::{ribbon_needed, wrapping_paper};
use md5::{Digest, Md5};
fn main() {
    let mut hasher = Md5::new();
    let key = find_hash("./inputs/day4.txt");
    hasher.update(&key.as_bytes());
    let result = format!("{:x}", hasher.finalize());
    println!("{} -> {}", key, result);
}
