mod day5;

fn main() {
    let s = include_str!("data\\day5-input.txt");
    println!("Part 1: {:?}", day5::part1(s));
    println!("Part 2: {:?}", day5::part2(s));
}
