mod day13;

fn main() {
    let s = include_str!("data\\day13-input.txt");
    println!("Part 1: {:?}", day13::part1(s));
    println!("Part 2: {:?}", day13::part2(s));
}
