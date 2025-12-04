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

    #[allow(unused, reason = "runs within an order of magnitude of thel lifetime of the universe")]
    fn joltages_schlonger(&self) -> impl Iterator<Item = u64> {
        const MAX: u64 = 12;

        self.banks.iter().map(|bank| {
            let mut buf = String::new();
            let end = (bank.len() - 1) as u64;

            let mut indices: Vec<_> = (0..MAX).collect();

            indices.iter().for_each(|&i| write!(buf, "{}", bank[i as usize]).unwrap());

            let mut biggest = buf.parse().unwrap();

            // Brute force algo

            'outer:
            loop {
                for i in (0..MAX).rev() {
                    let iu = i as usize;
                    if indices[iu] == end {
                        continue;
                    }

                    let next = indices[iu] + 1;
                    if indices[iu..].contains(&next) {
                        continue;
                    }

                    indices[iu] = next;

                    for (j, u) in indices.iter_mut().enumerate().take(MAX as usize).skip(iu + 1) {
                        *u = next + j as u64 - i;
                    }

                    buf.clear();
                    indices.iter().for_each(|&i| write!(buf, "{}", bank[i as usize]).unwrap());

                    let parsed = buf.parse().unwrap();

                    // println!("{indices:?}");

                    if parsed > biggest {
                        println!("new biggest: {parsed} {indices:?}");
                        biggest = parsed;
                    }

                    continue 'outer;
                }

                println!("finished one loop {biggest}");

                break;
            }

            biggest
        })
    }

    fn joltages_smarter(&self) -> impl Iterator<Item = u64> {
        const MAX: usize = 12;

        fn explore_range(buf: &mut String, ibuf: &mut Vec<usize>, bank: &[u8], start: usize, remaining: usize) -> u64 {
            if remaining == 0 {
                buf.clear();
                ibuf.iter().for_each(|&i| write!(buf, "{}", bank[i]).unwrap());

                // eprintln!("ending {ibuf:?} {buf}");

                return buf.parse().unwrap();
            }

            // eprintln!("{start} {remaining} {:?}", start..=(bank.len() - remaining));

            let max_value = bank[start..=(bank.len() - remaining)].iter().cloned().max().unwrap();

            let max_indices = bank.iter().enumerate()
                .skip(start)
                .filter_map(|(i, n)| (*n == max_value && i + remaining <= bank.len()).then_some(i));

            // assert!(!max_indices.is_empty());

            let mut biggest = 0;

            for max_index in max_indices {
                ibuf.push(max_index);

                let result = explore_range(buf, ibuf, bank, max_index + 1, remaining - 1);

                if result > biggest {
                    biggest = result;
                }

                ibuf.pop();
            }

            biggest
        }

        self.banks.iter().map(|bank| {
            let mut buf = String::new();
            let mut ibuf = Vec::new();

            explore_range(&mut buf, &mut ibuf, bank, 0, MAX)
        })
    }
}

fn part1() {
    let lobby = Lobby::parse(INPUT);

    dbg!(lobby.joltages().sum::<u32>());
}

fn part2() {
    let lobby = Lobby::parse(INPUT);

    dbg!(lobby.joltages_smarter().sum::<u64>());
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

        // assert_eq!(lobby.joltages().collect::<Vec<_>>(), vec![98, 89, 78, 92]);
        // assert_eq!(lobby.joltages().sum::<u32>(), 357);
    }

    #[test]
    fn example_part2() {
        let lobby = Lobby::parse(EXAMPLE);

        assert_eq!(lobby.joltages_smarter().collect::<Vec<_>>(), vec![987654321111, 811111111119, 434234234278, 888911112111]);
        assert_eq!(lobby.joltages_smarter().sum::<u64>(), 3121910778619);
    }
}
