# Advent of code 2020

Solutions for the [2020 Advent of code](https://adventofcode.com/2020), as written by an inveterate C# fanboy.

Enivornment:

- [Rust](https://www.rust-lang.org/)
- Windows
- [VSCode](https://code.visualstudio.com/) + [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer)

## Project layout

Each of the 25 advent of code puzzles has two parts, which both use the same input. (I assume — at time of writing I'm not done yet.)

I'm making each part a separate rust `mod` — in practice a separate `dayXX.rs` file — and editing the `main.rs` to import only "today's" mod and invoke its methods. Pulling the `day-X` branch and `cargo run` will write the correct solution to both parts of that day's puzzle.

Typically:

- the input will be a text file, downloaded unedited from the advent of code website `/2020/day/XX/input`
- the input is provided to the `dayXX` mod implementation using `include_str!`

However, occasionally part 2 is so different that I'll replace the implementation of `pub fn part1` with just `println!(answer)`, so I don't have to do tedious maintenance.

## House rules

None, really. In particular, any library is OK, very occasionally dipping in to fasterthanli.me for some inspiration.

I'm not going for the fastest or most elegant solutions. I'm trying to write idiomatic rust, correctly, as a learning exercise.
