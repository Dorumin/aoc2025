use std::{collections::HashSet, fmt::Write, ops::RangeInclusive};

const INPUT: &str = include_str!("../inputs/day2.txt");

struct Elfilter {
    ranges: Vec<RangeInclusive<u64>>
}

/// I wish I could use doctests in binaries
fn split_pieces(n: u64, piece_digits: u32) -> impl Iterator<Item = u64> {
    assert!(n > 0 && piece_digits > 0);

    let number_digits = Elfilter::count_digits(n);
    assert!(number_digits.is_multiple_of(piece_digits));

    let piece_divisor = 10u64.pow(piece_digits);

    std::iter::successors(Some((number_digits - piece_digits) as i32), move |&d| {
        let next = d - piece_digits as i32;
        (next >= 0).then_some(next)
    })
    .map(move |digits_to_truncate| {
        (n / 10u64.pow(digits_to_truncate as u32)) % piece_divisor
    })
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

    fn normalize_ranges_log10(&mut self) {
        self.ranges = std::mem::take(&mut self.ranges).into_iter().flat_map(|r| {
            let s = r.start().ilog10();
            let e = r.end().ilog10();

            (s..=e).map(move |i| {
                let start = 10u64.pow(i);
                let end = 10u64.pow(i + 1) - 1;

                start.max(*r.start())..=end.min(*r.end())
            })
        }).collect();
    }

    fn count_digits(num: u64) -> u32 {
        num.ilog10() + 1
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

    #[allow(unused)]
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
            }
        }

        invalidsum
    }

    fn pux_sieve(&mut self) -> u64 {
        self.normalize_ranges_log10();

        let mut witness_me = HashSet::new();
        let mut invalidsum = 0;

        for range in self.ranges.iter() {
            let lower_bound = *range.start();
            let upper_bound = *range.end();

            assert!(1 <= lower_bound && lower_bound <= upper_bound);

            let bound_digits = Self::count_digits(lower_bound);

            assert_eq!(bound_digits, Self::count_digits(upper_bound));

            for piece_digits in 1..(bound_digits / 2 + 1) {
                if !bound_digits.is_multiple_of(piece_digits) {
                    continue
                }

                let num_pieces = bound_digits / piece_digits;
                let mut lower_bound_pieces = split_pieces(lower_bound, piece_digits);
                let mut lower_piece_bound = lower_bound_pieces.next().unwrap();

                for next_piece in lower_bound_pieces {
                    if lower_piece_bound == next_piece {
                        continue
                    }

                    if lower_piece_bound < next_piece {
                        lower_piece_bound += 1
                    }

                    break
                }

                let mut upper_bound_pieces = split_pieces(upper_bound, piece_digits);
                let mut upper_piece_bound = upper_bound_pieces.next().unwrap();

                for next_piece in upper_bound_pieces {
                    if upper_piece_bound == next_piece {
                        continue
                    }

                    if upper_piece_bound > next_piece {
                        upper_piece_bound -= 1
                    }

                    break
                }

                if lower_piece_bound > upper_piece_bound {
                    continue
                }

                let piece_divisor = 10u64.pow(piece_digits);
                let mut delta = 1;

                for _ in 0..(num_pieces - 1) {
                    delta = (delta * piece_divisor) + 1;
                }

                let mut invalid_product_id = delta * lower_piece_bound;

                assert!(lower_bound <= invalid_product_id && invalid_product_id <= upper_bound);

                if !witness_me.contains(&invalid_product_id) {
                    invalidsum += invalid_product_id;
                    witness_me.insert(invalid_product_id);
                }

                for _ in (lower_piece_bound + 1)..=upper_piece_bound {
                    invalid_product_id += delta;

                    assert!(lower_bound <= invalid_product_id && invalid_product_id <= upper_bound);
                    if !witness_me.contains(&invalid_product_id) {
                        invalidsum += invalid_product_id;
                        witness_me.insert(invalid_product_id);

                    }
                }
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
    let mut filter = Elfilter::parse(INPUT);

    dbg!(filter.pux_sieve());
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

    #[test]
    fn example_pux() {
        let mut filter = Elfilter::parse(EXAMPLE);

        assert_eq!(filter.pux_sieve(), 4174379265);
    }

    #[test]
    fn splits() {
        assert_eq!(split_pieces(123456, 1).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(split_pieces(123456, 2).collect::<Vec<_>>(), vec![12, 34, 56]);
        assert_eq!(split_pieces(123456, 3).collect::<Vec<_>>(), vec![123, 456]);
    }
}
