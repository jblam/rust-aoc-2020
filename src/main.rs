fn main() {
    println!("{}", day2::evaluate::<day2::PartOne>(include_str!("data\\day2-input.txt")));
    println!("{}", day2::evaluate::<day2::PartTwo>(include_str!("data\\day2-input.txt")));
}

mod day2;