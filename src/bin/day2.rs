use std::{fmt::Write, ops::RangeInclusive};

const INPUT: &str = include_str!("../inputs/day2.txt");

struct Elfilter {
    ranges: Vec<RangeInclusive<u64>>
}

impl Elfilter {
    fn parse(s: &str) -> Self {
        Self {
            ranges: s.split(',').map(|s| {
                let (first, last) = s.trim().split_once('-').unwrap();
                let (first, last) = (first.parse().unwrap(), last.parse().unwrap());

                first..=last
            }).collect()
        }
    }

    fn simple_sieve(&self) -> u64 {
        let mut buf = String::new();
        let mut invalidsum = 0;

        for range in self.ranges.iter() {
            for i in range.clone() {
                buf.clear();

                write!(&mut buf, "{i}").unwrap();

                if buf.len().is_multiple_of(2) {
                    let mid = buf.len() / 2;

                    if buf[..mid] == buf[mid..] {
                        invalidsum += i;
                    }
                }
            }
        }

        invalidsum
    }

    fn repeat_sieve(&self) -> u64 {
        let mut buf = String::new();
        let mut invalidsum = 0;

        for range in self.ranges.iter() {
            'numbers_loop:
            for n in range.clone() {
                buf.clear();

                write!(&mut buf, "{n}").unwrap();

                let slice = buf.as_bytes();

                'length_loop:
                for i in 0..(slice.len() / 2) {
                    let i = i + 1;

                    if slice.len().is_multiple_of(i) {
                        // let mid = slice.len() / i;
                        let mut chunks = slice.chunks_exact(i);
                        let start = chunks.next().unwrap();

                        for next in chunks {
                            if next != start {
                                continue 'length_loop;
                            }
                        }

                        invalidsum += n;

                        continue 'numbers_loop;
                    }
                }
                // if buf.len().is_multiple_of(2) {
                //     let mid = buf.len() / 2;

                //     if buf[..mid] == buf[mid..] {
                //         invalidsum += n;
                //     }
                // }
            }
        }

        invalidsum
    }
}

fn part1() {
    let filter = Elfilter::parse(INPUT);

    dbg!(filter.simple_sieve());
}

fn part2() {
    let filter = Elfilter::parse(INPUT);

    dbg!(filter.repeat_sieve());
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

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn example() {
        let filter = Elfilter::parse(EXAMPLE);

        assert_eq!(filter.simple_sieve(), 1227775554);
    }

    #[test]
    fn example_part2() {
        let filter = Elfilter::parse(EXAMPLE);

        assert_eq!(filter.repeat_sieve(), 4174379265);
    }
}
