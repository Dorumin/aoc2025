const INPUT: &str = include_str!("../inputs/day1.txt");

struct Unsafe {
    position: u8
}

struct Rotations {
    rotations: Vec<Rotation>
}

struct Rotation {
    direction: Direction,
    turns: u32
}

#[derive(PartialEq)]
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
            let mut pos = self.position as i32;
            let rot = rotation.turns as i32;

            pos += if rotation.direction == Direction::Left { -rot } else { rot };

            self.position = pos.rem_euclid(100)  as u8;

            if self.position == 0 {
                landings += 1;
            }
        }

        landings
    }


    fn count_zero_slides(&mut self, rotations: &Rotations) -> u64 {
        let mut slides = 0;

        for rotation in rotations.rotations.iter() {
            let mut pos = self.position as i32;
            let rot = rotation.turns as i32;

            pos += if rotation.direction == Direction::Left { -rot } else { rot };

            let loops = (pos / 100).abs() + (if pos <= 0 && self.position != 0 { 1 } else { 0 });

            self.position = pos.rem_euclid(100) as u8;

            slides += loops;
        }

        slides as u64
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

            Rotation { direction, turns }
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
    let mut safe = Unsafe::new();
    let rotations = Rotations::parse(INPUT);

    let count = safe.count_zero_slides(&rotations);

    dbg!(count);
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

const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn example() {
        let mut safe = Unsafe::new();
        let rotations = Rotations::parse(EXAMPLE);

        assert_eq!(safe.count_zero_landings(&rotations), 3);
    }

    #[test]
    fn example_part2() {
        let mut safe = Unsafe::new();
        let rotations = Rotations::parse(EXAMPLE);

        assert_eq!(safe.count_zero_slides(&rotations), 6);
    }
}
