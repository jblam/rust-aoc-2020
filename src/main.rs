mod util;
mod day7;

fn main() {
    let s = include_str!("data\\day7-input.txt");
    println!("Part 1: {:?}", day7::part1(s));
    println!("Part 2: {:?}", day7::part2(s));
}
