#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day1.txt");

fn part1() {
    todo!();
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

    const EXAMPLE: &str = "";

    #[test]
    fn example() {
        // todo!();
    }

    #[test]
    fn example_part2() {
        // todo!();
    }
}
