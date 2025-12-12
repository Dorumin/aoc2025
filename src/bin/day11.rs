use std::collections::HashMap;

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
}

fn part1() {
    let reactor = Hiroshima::parse(INPUT);

    dbg!(reactor.count_all_paths("you", "out"));
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

    #[test]
    fn example() {
        let reactor = Hiroshima::parse(EXAMPLE);

        assert_eq!(reactor.count_all_paths("you", "out"), 5);
    }

    #[test]
    fn example_part2() {
        // todo!();
    }
}
