mod util;
mod day14;

fn main() {
    let s = include_str!("data\\day14-input.txt");
    println!("Part 1: {:?}", day14::part1(s));
    println!("Part 2: {:?}", day14::part2(s));
}
