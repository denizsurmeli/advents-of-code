use std::hash;

use aoc2015::day_five::nice_words;
use aoc2015::day_four::find_hash;
use aoc2015::day_one::not_quite_list_p2;
use aoc2015::day_seven::Emulator;
use aoc2015::day_six::build_decoration;
use aoc2015::day_three::house_visits_with_robot;
use aoc2015::day_two::{ribbon_needed, wrapping_paper};

fn main() {
    let mut emulator = Emulator::new("./inputs/day7.txt");
    emulator.execute_circuit();
    println!("{:?}",emulator.get_wire("a"));
}
//split into pairs, then count the "parts".
