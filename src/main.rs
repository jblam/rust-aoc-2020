mod util;
mod day8;

fn main() {
    let s = include_str!("data\\day8-input.txt");
    println!("Part 1: {:?}", day8::part1(s));
    println!("Part 2: {:?}", day8::part2(s));
}
