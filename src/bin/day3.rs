use std::fmt::Write;

const INPUT: &str = include_str!("../inputs/day3.txt");

struct Lobby {
    banks: Vec<Vec<u8>>
}

impl Lobby {
    fn parse(s: &str) -> Self {
        Self {
            banks: s.lines()
                .map(|line|
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                )
                .collect()
        }
    }

    fn joltages(&self) -> impl Iterator<Item = u32> {
        self.banks.iter().map(|bank| {
            // Let's keep it simple, obviously it'd start with the largest digit and the max length

            let mut buf = String::new();
            let mut biggest = 0;

            for i in 0..bank.len() {
                for j in (i + 1)..bank.len() {
                    let a = bank[i];
                    let b = bank[j];

                    buf.clear();

                    write!(buf, "{a}{b}").unwrap();

                    let c = buf.parse().unwrap();

                    if c > biggest {
                        biggest = c;
                    }
                }
            }

            biggest
        })
    }
}

fn part1() {
    let lobby = Lobby::parse(INPUT);

    dbg!(lobby.joltages().sum::<u32>());
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

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn example() {
        let lobby = Lobby::parse(EXAMPLE);

        assert_eq!(lobby.joltages().collect::<Vec<_>>(), vec![98, 89, 78, 92]);
        assert_eq!(lobby.joltages().sum::<u32>(), 357);
    }

    #[test]
    fn example_part2() {
        // todo!();
    }
}
