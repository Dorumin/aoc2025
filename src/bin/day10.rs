use rayon::iter::{IndexedParallelIterator as _, IntoParallelRefIterator as _, ParallelIterator as _};
use good_lp::{
    Expression, Solution, SolverModel, default_solver, variable, variables
};

#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day10.txt");

#[derive(Debug)]
struct Factory {
    machines: Vec<Machine>
}

#[derive(Debug)]
struct Machine {
    target_indicator_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl Factory {
    fn parse(s: &str) -> Self {
        let machines = s.lines().map(Machine::from_line).collect();

        Self {
            machines
        }
    }

    fn sum_of_fewest_presses(&self) -> usize {
        self.machines.iter().map(|m| m.fewest_presses()).inspect(|c| println!("{c}")).sum()
    }

    #[allow(unused)]
    fn sum_of_fewest_joltages(&self) -> usize {
        self.machines.par_iter()
            .enumerate()
            .map(|(index, m)| (index, m.fewest_presses_for_joltage_dumb()))
            .inspect(|(index, count)| println!("{index}: {count}"))
            .map(|(_, count)| count)
            .sum()
    }

    #[allow(unused)]
    fn sum_of_fewest_joltages_less_dumb(&self) -> usize {
        self.machines.par_iter()
            .enumerate()
            .map(|(index, m)| (index, m.fewest_presses_for_joltage_less_dumb()))
            .inspect(|(index, count)| println!("{index}: {count}"))
            .map(|(_, count)| count)
            .sum()

    }

    #[allow(unused)]
    fn sum_of_fewest_joltages_more_dumb(&self) -> usize {
        self.machines.iter()
            .enumerate()
            .map(|(index, m)| (index, m.fewest_presses_for_joltage_possibly_more_dumb()))
            .map(|(_, count)| count)
            .sum()

    }
}

impl Machine {
    fn from_line(s: &str) -> Self {
        // First non-trivial parsing problem
        // but I still got it first try B)
        let (_, s) = s.split_once('[').unwrap();
        let (pattern, mut s) = s.split_once("] (").unwrap();

        let mut buttons = vec![];
        let mut numbers = vec![];
        loop {
            let a = s.trim_start_matches(|c: char| c.is_ascii_digit());
            numbers.push(s[0..(s.len() - a.len())].parse().unwrap());

            s = a;

            if s.starts_with(',') {
                s = &s[1..];
            } else if s.starts_with(") {") {
                s = &s[3..];
                buttons.push(std::mem::take(&mut numbers));
                break;
            } else if s.starts_with(") (") {
                s = &s[3..];
                buttons.push(std::mem::take(&mut numbers));
            }
        }

        let mut joltage_requirements = vec![];

        loop {
            let a = s.trim_start_matches(|c: char| c.is_ascii_digit());
            joltage_requirements.push(s[0..(s.len() - a.len())].parse().unwrap());

            s = a;

            if s.starts_with(',') {
                s = &s[1..];
            } else if s.starts_with("}") {
                break;
            }
        }

        let target_indicator_lights = pattern.chars().map(|c| c == '#').collect();

        Self {
            target_indicator_lights,
            buttons,
            joltage_requirements,
        }
    }

    fn fewest_presses(&self) -> usize {
        let mut lights = vec![false; self.target_indicator_lights.len()];

        for count in 1.. {
            let mut indices = vec![0usize; count];

            loop {
                lights.fill(false);

                for bindex in indices.iter() {
                    for lindex in self.buttons[*bindex].iter() {
                        lights[*lindex] = !lights[*lindex];
                    }
                }

                // eprintln!("{indices:?} {lights:?}");

                if lights == self.target_indicator_lights {
                    eprintln!("{indices:?}");

                    return count;
                }

                let Some((index, _)) = indices.iter().enumerate().rev().find(|(_, count)| **count != self.buttons.len() - 1) else {
                    break;
                };

                indices[index] += 1;

                indices.iter_mut().skip(index + 1).for_each(|c| *c = 0);
            }

            if count > 50 {
                panic!();
            }
        }

        unreachable!();
    }

    #[allow(unused)]
    fn fewest_presses_for_joltage_dumb(&self) -> usize {
        let mut joltages = vec![0; self.joltage_requirements.len()];

        for count in 1.. {
            let mut indices = vec![0usize; count];

            loop {
                joltages.fill(0);

                for bindex in indices.iter() {
                    for lindex in self.buttons[*bindex].iter() {
                        joltages[*lindex] += 1;
                    }
                }

                if joltages == self.joltage_requirements {
                    // eprintln!("{indices:?}");

                    return count;
                }

                let Some((index, _)) = indices.iter().enumerate().rev().find(|(_, count)| **count != self.buttons.len() - 1) else {
                    break;
                };

                indices[index] += 1;

                indices.iter_mut().skip(index + 1).for_each(|c| *c = 0);
            }
        }

        unreachable!();
    }

    #[allow(unused)]
    fn fewest_presses_for_joltage_less_dumb(&self) -> usize {
        // Let's be less retarded; working backwards is pretty much the first step in most aoc optimization problems
        // The joltage requirements can only be reached by pressing some of the buttons,
        // so we can count which ones are needed to reach the requirements
        // Alternatively, we can be greedy and start by maximizing the buttons that produce the most joltages

        fn search(
            buttons: &[(usize, &Vec<usize>)],
            start: usize,
            remaining: &mut [usize],
            current: &mut Vec<usize>,
            best: &mut Option<usize>,
        ) {
            if remaining.iter().all(|&x| x == 0) {
                let presses = current.len();

                if best.is_none_or(|b| presses < b) {
                    *best = Some(presses);
                }

                return;
            }

            if let Some(b) = *best && current.len() >= b {
                return;
            }

            'buttons_loop:
            for button_index in start..buttons.len() {
                let (btn_index, idxs) = buttons[button_index];

                if !idxs.iter().all(|&idx| remaining[idx] > 0) {
                    continue;
                }

                for (idx_idx, &idx) in idxs.iter().enumerate() {
                    let Some(newb) = remaining[idx].checked_sub(1) else {
                        // Undo damage
                        for &idx in idxs.iter().take(idx_idx) {
                            remaining[idx] += 1;
                        }

                        continue 'buttons_loop;
                    };

                    remaining[idx] = newb;
                }

                current.push(btn_index);

                search(buttons, button_index, remaining, current, best);

                current.pop();
                for &idx in idxs {
                    remaining[idx] += 1;
                }
            }
        }

        let mut buttons: Vec<_> = self.buttons
            .iter()
            .enumerate()
            .collect();

        buttons.sort_by_key(|(_, idxs)| std::cmp::Reverse(idxs.len()));

        let mut best = None;
        let mut current = Vec::new();
        let mut remaining = self.joltage_requirements.clone();

        search(&buttons, 0, &mut remaining, &mut current, &mut best);

        best.unwrap()
    }

    fn fewest_presses_for_joltage_possibly_more_dumb(&self) -> usize {
        // I had this idea while taking a shit: what if we represent the joltages as an n-dimensional position
        // And we can just do a graph search where each step can bring us closer to that position?
        // Really this means implementing fucking dijkstra again but I'm done with that and I brought in
        // a lib to use A* with

        #[derive(PartialEq, Eq, Hash, Clone, Debug)]
        struct Position {
            remaining_joltages: Vec<usize>,
            cost: usize
        }

        impl Position {
            fn successors(&self, machine: &Machine) -> Vec<(Position, usize)> {
                let mut result = Vec::new();

                'outer:
                for affected in machine.buttons.iter() {
                    let mut next = self.remaining_joltages.clone();

                    for &i in affected {
                        if next[i] == 0 {
                            continue 'outer;
                        }

                        next[i] -= 1;
                    }

                    result.push((
                        Position {
                            cost: self.cost - affected.len(),
                            remaining_joltages: next,
                        },
                        // All the costs are 1 (as is the heuristic)
                        // Idk if this fucks over A*
                        1,
                    ));
                }

                // eprintln!("{result:?}");

                // panic!();

                result
            }

            fn heuristic(&self) -> usize {
                // In the worst case, you'd press a button for every single joltage remaining, wouldn't you?
                self.cost
            }
        }

        let result = pathfinding::directed::astar::astar(
        &Position {
                remaining_joltages: self.joltage_requirements.clone(),
                cost: self.joltage_requirements.iter().sum()
            },
            |p| p.successors(self),
            |p| p.heuristic(),
            |p| p.remaining_joltages.iter().all(|j| *j == 0)
        );

        result.unwrap().1
    }

    pub fn fewest_presses_linalg_solver(&self) -> usize {
        let n = self.joltage_requirements.len();

        let mut vars = variables!();
        let x: Vec<_> = (0..self.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let mut problem = vars.minimise(x.iter().sum::<Expression>()).using(default_solver);

        for i in 0..n {
            let mut lhs = Expression::from(0);

            for (button, var) in self.buttons.iter().zip(&x) {
                if button.contains(&i) {
                    lhs += var;
                }
            }

            // note: might need to use epsilon distance because of f64 limitation; worked on my input
            problem = problem.with(lhs.eq(self.joltage_requirements[i] as f64));
        }

        let solution = problem.solve().unwrap();

        // Round because fuckass linalg solver doesn't support true integers and otherwise it'll trunc
        x.iter().map(|var| solution.value(*var).round() as usize).sum()
    }
}

fn part1() {
    let factory = Factory::parse(INPUT);

    dbg!(factory.sum_of_fewest_presses());
}

fn part2() {
    let factory = Factory::parse(INPUT);

    dbg!(Machine::from_line("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").fewest_presses_linalg_solver());
    dbg!(Machine::from_line("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}").fewest_presses_linalg_solver());
    dbg!(Machine::from_line("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}").fewest_presses_linalg_solver());

    // dbg!(factory.sum_of_fewest_joltages_less_dumb());
    dbg!(factory.machines.iter().fold(0, |sum, m| sum + m.fewest_presses_linalg_solver()));

    // factory.machines.iter()
    //     .enumerate()
    //     .map(|(index, m)| (index, m.fewest_presses_linalg_solver()))
    //     .inspect(|(index, count)| println!("{index}: {count}"))
    //     .map(|(_, count)| count)
    //     .for_each(|_| {});

    // dbg!(factory.machines[0].fewest_presses_fucking_linalg_solver());
    // dbg!(factory.machines[124].fewest_presses_fucking_linalg_solver());
    // dbg!(factory.machines[0].fewest_presses_for_joltage_possibly_more_dumb());
    // dbg!(factory.machines[1].fewest_presses_for_joltage_possibly_more_dumb());
    // dbg!(factory.machines[2].fewest_presses_for_joltage_possibly_more_dumb());
    // // dbg!(factory.machines[0].fewest_presses_for_joltage_less_dumb());
    // dbg!(factory.sum_of_fewest_joltages_it_wasnt_more_dumb());
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

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    // #[test]
    // fn example() {
    //     let factory = Factory::parse(EXAMPLE);

    //     assert_eq!(factory.sum_of_fewest_presses(), 7);
    // }

    #[test]
    fn example_part2() {
        let factory = Factory::parse(EXAMPLE);

        assert_eq!(factory.machines[0].fewest_presses_for_joltage_less_dumb(), 10);
        // assert_eq!(factory.sum_of_fewest_joltages_less_dumb(), 33);
        // assert_eq!(factory.sum_of_fewest_joltages(), 33);
        assert_eq!(factory.sum_of_fewest_joltages_more_dumb(), 33);

        panic!();
    }
}
