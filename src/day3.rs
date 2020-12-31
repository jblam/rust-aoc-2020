use std::ops::Index;

struct Vec2 {
    x: i32,
    y: i32,
}
impl From<(i32, i32)> for Vec2 {
    fn from(t: (i32, i32)) -> Self {
        Vec2 { x: t.0, y: t.1 }
    }
}
struct TravelMap {
    width: usize,
    trees: Vec<Vec<(usize, Tile)>>,
}
impl TravelMap {
    fn parse(s: &str) -> TravelMap {
        let width = s.lines().next().map_or(1, str::len);
        let trees = s
            .lines()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| **b == b'#')
                    .map(|(col, _)| (col, Tile::Tree))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        TravelMap { width, trees }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Tree,
}


impl Index<Vec2> for TravelMap {
    type Output = Tile;

    fn index(&self, index: Vec2) -> &Self::Output {
        if index.y < 0 || index.y as usize >= self.trees.len() {
            &Tile::Empty
        } else {
            let col = index.x % self.width as i32;
            let col = if col < 0 { col + self.width as i32 } else { col } as usize;
            self.trees[index.y as usize]
                .iter()
                .find(|(tree_col, _)| col == *tree_col)
                .map_or(&Tile::Empty, |(_, t)| t)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    #[test]
    fn can_parse() {
        let m = TravelMap::parse(EXAMPLE);
        assert_eq!(11, m.width);
        assert_eq!(11, m.trees.len());
        assert_eq!([0usize, 4, 8].iter().map(|i| (*i, Tile::Tree)).collect::<Vec<_>>(), m.trees[1])
    }
}
