mod day15;

fn main() {
    // let s = include_str!("data\\day14-input.txt");
    let sequence = [0, 20, 7, 16, 1, 18, 15];
    println!("Part 1: {:?}", day15::part1(&sequence));
    println!("Part 2: {:?}", day15::part2(&sequence));
}
