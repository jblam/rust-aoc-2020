mod util;
mod day16;

fn main() {
    let s = include_str!("data\\day16-input.txt");
    println!("Part 1: {:?}", day16::part1(s));
    println!("Part 2: {:?}", day16::part2(s));
}
