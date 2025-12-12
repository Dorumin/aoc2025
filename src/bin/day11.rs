use std::collections::HashMap;

use cached::proc_macro::cached;

#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day11.txt");

struct Hiroshima {
    connections: HashMap<&'static str, Vec<&'static str>>
}

impl Hiroshima {
    fn parse(s: &'static str) -> Self {
        let mut connections = HashMap::new();

        s.lines().for_each(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            let c = b.split(' ').collect();

            connections.insert(a, c);
        });

        Self {
            connections
        }
    }

    fn count_all_paths(&self, from: &'static str, to: &'static str) -> usize {
        let mut stack = Vec::new();

        fn search(hiroshima: &Hiroshima, stack: &mut Vec<&'static str>, end: &'static str, current: &'static str) -> usize {
            if current == end {
                return 1;
            }

            let Some(nexts) = hiroshima.connections.get(current) else {
                eprintln!("missing: {current} {end}");

                return 0;
            };
            let mut sum = 0;

            for next in nexts {
                stack.push(next);

                sum += search(hiroshima, stack, end, next);

                stack.pop();
            }

            sum
        }


        search(self, &mut stack, to, from)
    }

    fn count_all_paths_nanalog(&self, from: &'static str, to: &'static str) -> usize {
        // DP hard, DP often. But especially hard
        #[cached(key = "(&'static str, bool, bool)", convert = "{ (current, seen_dac, seen_fft) }")]
        fn search_cached(
            hiroshima: &Hiroshima,
            end: &'static str,
            current: &'static str,
            seen_dac: bool,
            seen_fft: bool
        ) -> usize {
            if current == end {
                if seen_dac && seen_fft {
                    return 1;
                } else {
                    return 0;
                }
            }

            let seen_dac = seen_dac || current == "dac";
            let seen_fft = seen_fft || current == "fft";

            let nexts = match hiroshima.connections.get(current) {
                Some(n) => n,
                None => return 0,
            };

            let mut sum = 0;
            for next in nexts {
                sum += search_cached(hiroshima, end, next, seen_dac, seen_fft);
            }

            sum
        }

        search_cached(self, to, from, false, false)
    }
}

fn part1() {
    let reactor = Hiroshima::parse(INPUT);

    dbg!(reactor.count_all_paths("you", "out"));
}

fn part2() {
    let reactor = Hiroshima::parse(INPUT);

    // dbg!(reactor.count_all_paths("svr", "out")); real cute making part 1 a path without quadrillions of routes
    dbg!(reactor.count_all_paths_nanalog("svr", "out"));
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

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn example() {
        let reactor = Hiroshima::parse(EXAMPLE);

        assert_eq!(reactor.count_all_paths("you", "out"), 5);
    }

    #[test]
    fn example_part2() {
        let reactor = Hiroshima::parse(EXAMPLE2);

        assert_eq!(reactor.count_all_paths("svr", "out"), 8);
        assert_eq!(reactor.count_all_paths_nanalog("svr", "out"), 2);
    }
}
