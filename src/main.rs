fn main() {
    let s = include_str!("data\\day4-input.txt");
    println!("Part 1: {:?}", day4::part1(s));
    println!("Part 2: {:?}", day4::part2(s));
}

mod day4;