use std::hash;

use aoc2015::day_five::nice_words;
use aoc2015::day_four::find_hash;
use aoc2015::day_one::not_quite_list_p2;
use aoc2015::day_six::build_decoration;
use aoc2015::day_three::house_visits_with_robot;
use aoc2015::day_two::{ribbon_needed, wrapping_paper};

fn main() {
    println!("{}", build_decoration("./inputs/day6.txt"));
}
//split into pairs, then count the "parts".
