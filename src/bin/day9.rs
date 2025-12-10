use std::{collections::HashSet, fmt::{Display, Write}, io::BufWriter};
use std::io::Write as _;

#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day9.txt");

struct BijouTheater {
    red_tiles: Vec<(u32, u32)>,
    set: HashSet<(u32, u32)>,
    max: (u32, u32)
}

impl BijouTheater {
    fn parse(s: &str) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut set = HashSet::new();

        Self {
            red_tiles: s.lines().map(|line| {
                let mut split = line.split(',');
                let (Some(x), Some(y)) = (split.next(), split.next()) else {
                    panic!("Invalid line");
                };
                let (Ok(x), Ok(y)) = (x.parse(), y.parse()) else {
                    panic!("Invalid line but in a different way");
                };

                max_x = max_x.max(x);
                max_y = max_y.max(y);
                set.insert((x, y));

                (x, y)
            }).collect(),
            set,
            max: (max_x, max_y)
        }
    }

    fn try_all_rectangles(&self) -> u64 {
        let mut largest = 0;

        for x in self.red_tiles.iter() {
            for y in self.red_tiles.iter() {
                if x == y {
                    // Still no selfcest; this is combinations(2)
                    // Not that it would make a particularly large rectangule
                    continue;
                }

                // These rectangles are inclusive ranges
                let size = (x.0.abs_diff(y.0) as u64 + 1) * (x.1.abs_diff(y.1) as u64 + 1);

                if size > largest {
                    eprintln!("{size}");
                    largest = size;
                }
            }
        }

        largest
    }
}

impl Display for BijouTheater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=(self.max.1 + 1) {
            for x in 0..=(self.max.0 + 1) {
                let c = if self.set.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };

                f.write_char(c)?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn part1() {
    let theater = BijouTheater::parse(INPUT);
    let test = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("boob.txt")
        .unwrap();
    let mut test = BufWriter::new(test);

    // careful, this file is 9gb
    write!(&mut test, "{theater}").unwrap();

    dbg!(theater.try_all_rectangles());
}

fn part2() {
    todo!();
}

fn main() {
    let mut vargs = std::env::args().skip(1);

    match vargs.next().expect("Pass the part").parse().expect("It's a number") {
        1 => part1(),
        2 => part2(),
        _ => panic!("... between 1 and 2.")
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn example() {
        let theater = BijouTheater::parse(EXAMPLE);

        // Almost what appears in the readme - it has two spaces padded at the end for some reason
        assert_eq!(format!("{theater}"), ".............
.......#...#.
.............
..#....#.....
.............
..#......#...
.............
.........#.#.
.............
");


        assert_eq!(theater.try_all_rectangles(), 50);
    }

    #[test]
    fn example_part2() {
        // todo!();
    }
}
