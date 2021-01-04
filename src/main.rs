mod day10;

fn main() {
    let s = include_str!("data\\day10-input.txt");
    println!("Part 1: {:?}", day10::part1(s));
    println!("Part 2: {:?}", day10::part2(s));
}
