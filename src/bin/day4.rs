const INPUT: &str = include_str!("../inputs/day4.txt");

struct Printing {
    rolls: Vec<Vec<bool>>
}

impl Printing {
    fn parse(s: &str) -> Self {
        Self {
            rolls: s.lines().map(|line| line.chars().map(|c| c == '@').collect()).collect()
        }
    }

    fn count_accessible_rolls(&self) -> u32 {
        let mut count = 0;

        let height = self.rolls.len() as isize;
        let width = self.rolls[0].len() as isize;

        let adjacents = |x: usize, y: usize| {
            [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]
                .into_iter()
                .filter_map(move |(delta_x, delta_y)| {
                    let next = ((x as isize) + delta_x, (y as isize) + delta_y);

                    if next.0 < 0 || next.0 >= width || next.1 < 0 || next.1 >= height {
                        None
                    } else {
                        Some((next.0 as usize, next.1 as usize))
                    }
                })
        };

        for (y, row) in self.rolls.iter().enumerate() {
            for (x, is_roll) in row.iter().enumerate() {
                if *is_roll {
                    let adjacent_count = adjacents(x, y).filter(|(x, y)| self.rolls[*y][*x]).count();

                    if adjacent_count < 4 {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn part1() {
    let printing = Printing::parse(INPUT);

    dbg!(printing.count_accessible_rolls());
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

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn example() {
        let printing = Printing::parse(EXAMPLE);

        assert_eq!(printing.count_accessible_rolls(), 13);
    }

    #[test]
    fn example_part2() {
        // todo!();
    }
}
