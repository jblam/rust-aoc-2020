mod day6;

fn main() {
    let s = include_str!("data\\day6-input.txt");
    println!("Part 1: {:?}", day6::part1(s));
    println!("Part 2: {:?}", day6::part2(s));
}
