use std::collections::HashSet;

#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day6.txt");

struct Cephalopostulate<'a> {
    // Keeping as unfancy strings because pux spoiled fuckery in the parsing
    rows: Vec<Vec<&'a str>>
}

impl<'a> Cephalopostulate<'a> {
    fn parse(s: &'a str) -> Self {
        // You won't catch me dead trying to do it in one pass
        let mut whitespace_indices = HashSet::new();
        let mut non_whitespace_indices = HashSet::new();

        for line in s.lines() {
            line.char_indices().for_each(|(i, c)| {
                if c.is_ascii_whitespace() {
                    whitespace_indices.insert(i);
                } else {
                    non_whitespace_indices.insert(i);
                }
            });
        }

        let mut spacers: Vec<_> = whitespace_indices.difference(&non_whitespace_indices).cloned().collect();
        spacers.sort();

        Self {
            rows: s.lines().map(|line| {
                let mut sections = vec![];
                let mut start = 0;

                for spacer in spacers.iter().cloned() {
                    sections.push(&line[start..spacer]);

                    start = spacer + 1;
                }

                if line.len() > start {
                    sections.push(&line[start..line.len()]);
                }

                sections
            }).collect()
        }
    }

    fn solved(&self) -> impl Iterator<Item = u64> {
        let length = self.rows[0].len();

        (0..length).flat_map(|cell_index| {
            let op = self.rows.last().unwrap()[cell_index].trim();

            (0..(self.rows.len() - 1)).map(|row_index| {
                self.rows[row_index][cell_index].trim().parse().unwrap()
            }).reduce(|acc, num| if op == "*" {
                acc * num
            } else {
                acc + num
            })
        })
    }
}

fn part1() {
    let ceph = Cephalopostulate::parse(INPUT);

    dbg!(ceph.solved().sum::<u64>());
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

    const EXAMPLE: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn example() {
        let ceph = Cephalopostulate::parse(EXAMPLE);

        assert_eq!(ceph.solved().collect::<Vec<_>>(), vec![
            33210,
            490,
            4243455,
            401,
        ]);
    }

    #[test]
    fn example_part2() {
        // todo!();
    }
}
