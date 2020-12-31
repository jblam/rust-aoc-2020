fn main() {
    let s = include_str!("data\\day3-input.txt");
    println!("Part 1: {} trees", day3::part1(s));
    println!("Part 2: {} trees²", day3::part2(s));
}

mod day3;