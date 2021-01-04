mod day11;

fn main() {
    let s = include_str!("data\\day11-input.txt");
    println!("Part 1: {:?}", day11::part1(s));
    println!("Part 2: {:?}", day11::part2(s));
}
