use std::{collections::HashSet, ops::RangeInclusive};

#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day5.txt");

struct Cafeteria {
    ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>
}

impl Cafeteria {
    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let mut ranges = Vec::new();
        let mut ingredients = Vec::new();

        loop {
            match lines.next() {
                Some("") => break,
                Some(line) => {
                    let (start, end) = line.split_once('-').unwrap();
                    let (start, end) = (start.parse().unwrap(), end.parse().unwrap());

                    ranges.push(start..=end);
                }
                None => panic!("But we haven't even gotten to the ingredients!"),
            }
        }

        for ingredient in lines {
            ingredients.push(ingredient.parse().unwrap());
        }

        Cafeteria { ranges, ingredients }
    }

    fn fresh_ingredient_count(&self) -> usize {
        self.ingredients.iter().filter(|ing| self.ranges.iter().any(|range| range.contains(ing))).count()
    }

    fn flatten_ranges(&self) -> HashSet<RangeInclusive<usize>> {
        let mut flattened_ranges = HashSet::new();
        let mut edges: Vec<_> = self.ranges.iter()
            .flat_map(|r| [*r.start(), *r.start() - 1, *r.end() - 1, *r.end()])
            .collect();

        edges.sort();

        // 1-10
        // 3-4
        // -> 1-2, 3-4, 5-10
        // 10-20
        // 5-12
        // -> 5-9, 10-12, 13-20
        // 1-2
        // 1-2

        // Actually, I don't know how it works

        for range in &self.ranges {
            let mut start = *range.start();
            for edge in edges.iter().cloned() {
                if range.contains(&edge) && start <= edge {
                    let flat = start..=edge;
                    // dbg!(&flat);
                    flattened_ranges.insert(flat);
                    start = edge + 1;
                }
            }

            if start <= *range.end() {
                flattened_ranges.insert(start..=*range.end());
            }
        }

        flattened_ranges
    }

    fn all_fresh_count(&self) -> usize {
        let flattened_ranges = self.flatten_ranges();

        flattened_ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    }
}

fn part1() {
    let cafeteria = Cafeteria::parse(INPUT);

    dbg!(cafeteria.fresh_ingredient_count());
}

fn part2() {
    let cafeteria = Cafeteria::parse(INPUT);

    dbg!(cafeteria.all_fresh_count());
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

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn example() {
        let cafeteria = Cafeteria::parse(EXAMPLE);

        assert_eq!(cafeteria.ranges.len(), 4);
        assert_eq!(cafeteria.ingredients.len(), 6);
        assert_eq!(cafeteria.fresh_ingredient_count(), 3);
    }

    #[test]
    fn example_part2() {
        let cafeteria = Cafeteria::parse(EXAMPLE);

        dbg!(cafeteria.flatten_ranges());

        assert_eq!(cafeteria.all_fresh_count(), 14);
    }
}
