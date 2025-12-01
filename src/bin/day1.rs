const INPUT: &str = include_str!("../inputs/day1.txt");


struct Unsafe {
    position: u8
}

struct Rotations {
    rotations: Vec<Rotation>
}

struct Rotation {
    direction: Direction,
    turns: u8
}

enum Direction {
    Left,
    Right
}

impl Unsafe {
    fn new() -> Self {
        Self {
            position: 50
        }
    }

    fn count_zero_landings(&mut self, rotations: &Rotations) -> u64 {
        let mut landings = 0;

        for rotation in rotations.rotations.iter() {
            self.position = match rotation.direction {
                Direction::Left => (self.position + 100 - rotation.turns) % 100,
                Direction::Right => (self.position + rotation.turns) % 100,
            };

            if self.position == 0 {
                landings += 1;
            }
        }

        landings
    }
}

impl Rotations {
    fn parse(s: &str) -> Self {
        let lines: Vec<_> = s.lines().map(|line| {
            let direction = match line.chars().next() {
                Some('L') => Direction::Left,
                Some('R') => Direction::Right,
                _ => panic!()
            };
            let turns: u32 = line[1..].parse().unwrap();
            let turns = turns % 100;

            Rotation { direction, turns: turns.try_into().unwrap() }
        }).collect();

        Self {
            rotations: lines
        }
    }
}

fn part1() {
    let mut safe = Unsafe::new();
    let rotations = Rotations::parse(INPUT);

    let count = safe.count_zero_landings(&rotations);

    dbg!(count);
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
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut safe = Unsafe::new();
        let rotations = Rotations::parse("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82");

        assert_eq!(safe.count_zero_landings(&rotations), 3);
    }
}
