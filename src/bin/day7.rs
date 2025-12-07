use std::fmt::Write;

#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day7.txt");

struct TheScientist {
    rows: Vec<Vec<Cell>>
}

enum Cell {
    Start,
    Beam(usize),
    Splitter,
    Free
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            'S' => Cell::Start,
            '|' => Cell::Beam(1),
            '^' => Cell::Splitter,
            '.' => Cell::Free,
            _ => unreachable!()
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cell::Start => 'S',
            Cell::Beam(_) => '|',
            Cell::Splitter => '^',
            Cell::Free => '.'
        }
    }
}

impl TheScientist {
    fn parse(s: &str) -> Self {
        Self {
            rows: s.lines().map(|line| line.chars().map(Cell::from_char).collect()).collect()
        }
    }

    fn step(&mut self) -> (usize, usize) {
        let mut targets = vec![];
        let mut sliced = 0;
        let mut splits = 0;

        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let raycast = matches!(cell, Cell::Start | Cell::Beam(_));

                if !raycast {
                    continue;
                }

                if let Some(cell) = self.rows.get(y + 1).and_then(|row| row.get(x)) {
                    if matches!(cell, Cell::Splitter) {
                        let mut next = vec![];
                        x.checked_sub(1).into_iter().for_each(|x| next.push((x, y + 1)));
                        x.checked_add(1).into_iter().for_each(|x| next.push((x, y + 1)));

                        if next.iter().cloned().any(|(x, y)| {
                            let tgt = self.rows.get(y).and_then(|row| row.get(x));

                            matches!(tgt, Some(Cell::Free))
                        }) {
                            splits += 1;
                        }

                        targets.extend(next);
                    } else {
                        targets.push((x, y + 1));
                    }
                } else {
                    targets.push((x, y + 1));
                }
            }
        }

        for (x, y) in targets {
            if let Some(cell) = self.rows.get_mut(y).and_then(|row| row.get_mut(x))
                && matches!(cell, Cell::Free) {
                *cell = Cell::Beam(1);

                sliced += 1;
            }
        }

        (sliced, splits)
    }

    fn stop(&mut self) -> usize {
        let mut splits_total = 0;

        loop {
            let (sliced, splits) = self.step();

            splits_total += splits;

            if sliced == 0 {
                break;
            }
        }

        splits_total
    }

    fn quantum_inferiority(&mut self) -> usize {
        for y in 0..(self.rows.len() - 1) {
            for x in 0..self.rows[y].len() {
                let row = &self.rows[y];
                let cell = &row[x];

                let stax = match cell {
                    Cell::Start => 1,
                    Cell::Beam(x) => *x,
                    _ => continue,
                };

                let nexts = match self.rows[y + 1][x] {
                    Cell::Splitter => vec![
                        (x.checked_sub(1), y + 1),
                        (x.checked_add(1), y + 1)
                    ],
                    Cell::Free | Cell::Beam(_) => vec![
                        (Some(x), y + 1)
                    ],
                    _ => unreachable!()
                };

                for next in nexts {
                    if let (Some(x), y) = next {
                        match &mut self.rows[y][x] {
                            Cell::Beam(stacks) => *stacks += stax,
                            cell @ Cell::Free  => *cell = Cell::Beam(stax),
                            _ => unreachable!()
                        }
                    }
                }
            }
        }

        self.rows.last().unwrap().iter().fold(0, |sum, cell| if let Cell::Beam(x) = cell { sum + x } else { sum })
    }
}

impl std::fmt::Display for TheScientist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for cell in row {
                f.write_char(cell.to_char())?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn part1() {
    let mut scientist = TheScientist::parse(INPUT);

    dbg!(scientist.stop());
}

fn part2() {
    let mut scientist = TheScientist::parse(INPUT);

    dbg!(scientist.quantum_inferiority());
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

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    const PUX: &str = ".S..
.^..
..^.
...^
....
.^..";

    #[test]
    fn example() {
        let mut scientist = TheScientist::parse(EXAMPLE);

        eprintln!("{scientist}");
        assert_eq!(scientist.stop(), 21);
    }

    #[test]
    fn test_patience_pux() {
        let mut scienpux = TheScientist::parse(PUX);

        eprintln!("{scienpux}");
        assert_eq!(scienpux.stop(), 4);
    }

    #[test]
    fn example_part2() {
        let mut scientist = TheScientist::parse(EXAMPLE);

        assert_eq!(scientist.quantum_inferiority(), 40);
    }
}
