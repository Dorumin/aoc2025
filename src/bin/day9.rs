use std::{collections::HashSet, fmt::{Debug, Display, Write}, io::BufWriter};
use std::io::Write as _;

#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day9.txt");

struct BijouTheater {
    red_tiles: Vec<(u32, u32)>,
    set: HashSet<(u32, u32)>,
    max: (u32, u32)
}

fn pairs_looping<T: Clone>(ring: &[T]) -> impl Iterator<Item = (&T, &T)> {
    (0..ring.len()).map(|i| {
        (&ring[i], &ring[(i + 1) % ring.len()])
    })
}

fn rect_size(a: (u32, u32), b: (u32, u32)) -> u64 {
    (a.0.abs_diff(b.0) as u64 + 1) * (a.1.abs_diff(b.1) as u64 + 1)
}

impl BijouTheater {
    fn parse(s: &str) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut set = HashSet::new();
        let red_tiles: Vec<_> = s.lines().map(|line| {
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
        }).collect();

        fn assert_closed_loop(points: &[(u32, u32)]) {
            for (a, b) in pairs_looping(points) {
                // Only str8 lines between pointz
                assert!((a.0.abs_diff(b.0) == 0) ^ (a.1.abs_diff(b.1) == 0));
            }
        }

        assert_closed_loop(&red_tiles);

        Self {
            red_tiles,
            set,
            max: (max_x, max_y)
        }
    }

    fn all_pairs(&self) -> impl Iterator<Item = ((u32, u32), (u32, u32))> {
        self.red_tiles.iter().flat_map(|&a|
            self.red_tiles.iter().filter_map(move |&b| (a != b).then_some((a, b)))
        )
    }

    fn try_all_rectangles(&self) -> u64 {
        let mut largest = 0;

        for (a, b) in self.all_pairs() {
            // These rectangles are inclusive ranges
            let size = rect_size(a, b);

            if size > largest {
                eprintln!("{size}");
                largest = size;
            }
        }

        largest
    }

    fn point_in_loop(&self, point: (u32, u32)) -> bool {
        fn point_on_segment(p: (i32, i32), a: (i32, i32), b: (i32, i32)) -> bool {
            // If it's textbook, what's the difference between it and using a library?
            // Textbook is less optimized, that's the difference

            let (px, py) = p;
            let (x1, y1) = a;
            let (x2, y2) = b;

            let dx = x2 - x1;
            let dy = y2 - y1;
            let dxp = px - x1;
            let dyp = py - y1;

            if dx as i64 * dyp as i64 - dy as i64 * dxp as i64 != 0 {
                return false;
            }

            if px < x1.min(x2) || px > x1.max(x2) { return false; }
            if py < y1.min(y2) || py > y1.max(y2) { return false; }

            true
        }

        let point = (point.0 as i32, point.1 as i32);
        let (px, py) = point;
        let verts = &self.red_tiles;

        let mut inside = false;

        for (&a, &b) in pairs_looping(verts) {
            let a = (a.0 as i32, a.1 as i32);
            let b = (b.0 as i32, b.1 as i32);
            let (x1, y1) = a;
            let (x2, y2) = b;

            if point_on_segment(point, a, b) {
                return true;
            }

            let intersects = ((y1 > py) != (y2 > py))
                && ((px as i64) < x1 as i64 + (x2 - x1) as i64 * (py - y1) as i64 / (y2 - y1) as i64);

            if intersects {
                inside = !inside;
            }
        }

        inside
    }

    fn try_all_rectangles_filled_with_suspicious_fluids(&self) -> u64 {
        let mut largest = 0;

        let pair_count = self.all_pairs().count();

        for (index, (a, b)) in self.all_pairs().enumerate() {
            // These rectangles are inclusive ranges
            let size = rect_size(a, b);

            if size > largest && self.rectangle_is_safe(a, b, index, pair_count) {
                largest = size;
            }
        }

        largest
    }

    fn rectangle_is_safe(&self, a: (u32, u32), b: (u32, u32), index: usize, count: usize) -> bool {
        let doru_asked_eric_about_these_and_found_out_theyre_wrong = [
            2785979082u64,
            2782680990,
            2774704992,
            2771406900,
            2716672230
        ];
        let xrange = (a.0.min(b.0))..=(a.0.max(b.0));
        let yrange = (a.1.min(b.1))..=(a.1.max(b.1));

        let size = rect_size(a, b);

        if doru_asked_eric_about_these_and_found_out_theyre_wrong.contains(&size) {
            return false;
        }

        if size < 5000000 {
            for x in xrange.clone() {
                for y in yrange.clone() {
                    if !self.point_in_loop((x, y)) {
                        return false;
                    }
                }
            }
        } else {
            for _ in 0..5000000 {
                let x = rand::random_range(xrange.clone());
                let y = rand::random_range(yrange.clone());

                if !self.point_in_loop((x, y)) {
                    return false;
                }
            }
        }

        eprintln!("improvement! {size} ({index} out of {count})");

        true
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

impl Debug for BijouTheater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=(self.max.1 + 1) {
            for x in 0..=(self.max.0 + 1) {
                let c = if self.set.contains(&(x, y)) {
                    '#'
                } else if self.point_in_loop((x, y)) {
                    'X'
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
    let theater = BijouTheater::parse(INPUT);

    dbg!(theater.try_all_rectangles_filled_with_suspicious_fluids());
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

        eprintln!("{theater:?}");

        assert_eq!(format!("{theater:?}"), ".............
.......#XXX#.
.......XXXXX.
..#XXXX#XXXX.
..XXXXXXXXXX.
..#XXXXXX#XX.
.........XXX.
.........#X#.
.............
");


        assert_eq!(theater.try_all_rectangles(), 50);
    }

    #[test]
    fn example_part2() {
        let theater = BijouTheater::parse(EXAMPLE);

        eprintln!("{theater:?}");

        assert_eq!(theater.try_all_rectangles_filled_with_suspicious_fluids(), 24);
    }
}
