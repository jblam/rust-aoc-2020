mod day12;

fn main() {
    let s = include_str!("data\\day12-input.txt");
    println!("Part 1: {:?}", day12::part1(s));
    println!("Part 2: {:?}", day12::part2(s));
}
