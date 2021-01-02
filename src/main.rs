mod util;
mod day9;

fn main() {
    let s = include_str!("data\\day9-input.txt");
    println!("Part 1: {:?}", day9::part1(s));
    println!("Part 2: {:?}", day9::part2(s));
}
