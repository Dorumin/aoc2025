use std::collections::{HashMap, HashSet};

#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day8.txt");

struct WeebsLastTour {
    boxes: Vec<Box>,
    last_circuit: usize,
    // circuits is a hashmap because they can appear and disappear when merging
    circuits: HashMap<usize, Circuit>,
    circuits_map: HashMap<usize, usize>,
    hitchings: HashSet<(usize, usize)>,
    nearest_neighbors: HashMap<usize, (usize, f64)>,
    nearest_neighbor: Option<(usize, f64)>
}

impl WeebsLastTour {
    fn parse(s: &str) -> Self {
        let boxes = s.lines().map(Box::from_line);

        let mut us = Self {
            boxes: boxes.collect(),
            last_circuit: 0,
            circuits: HashMap::new(),
            circuits_map: HashMap::new(),
            hitchings: HashSet::new(),
            nearest_neighbors: HashMap::new(),
            nearest_neighbor: None
        };

        us.precompute_nearest_neighbors();

        us
    }

    fn precompute_nearest_neighbors(&mut self) {
        // Mmm... tasty O(n^2)
        for i in 0..self.boxes.len() {
            self.compute_nearest_neighbors(i);
        }
    }

    fn compute_nearest_neighbors(&mut self, index: usize) {
        let mut nearest_match = None;
        let mut nearest_distance = f64::INFINITY;
        let a = &self.boxes[index];

        for (j, b) in self.boxes.iter().enumerate() {
            if index == j {
                // No selfcest
                continue;
            }

            if self.hitchings.contains(&(index, j)) {
                continue
            }

            let distance = a.distance_to(b);
            if distance < nearest_distance {
                nearest_match = Some(j);
                nearest_distance = distance;

                if let Some((idx, dist)) = &mut self.nearest_neighbor {
                    if distance < *dist {
                        *idx = index;
                        *dist = distance;
                    }
                } else {
                    self.nearest_neighbor = Some((index, distance));
                }
            }
        }

        self.nearest_neighbors.insert(index, (nearest_match.unwrap(), nearest_distance));
    }

    fn get_nearest_neighbor(&self) -> Option<(usize, f64)> {
        // I'm surprised I don't have a better way to compute this
        self.nearest_neighbors.values()
            .max_by(|(_, da), (_, db)| db.total_cmp(da))
            .cloned()
    }

    fn closest_bachelors(&self) -> Option<(usize, usize)> {
        let nearest_neighbor = self.get_nearest_neighbor()?.0;
        let neighbor = self.nearest_neighbors.get(&nearest_neighbor)?.0;

        // Sort for aid in tests
        if nearest_neighbor < neighbor {
            Some((nearest_neighbor, neighbor))
        } else {
            Some((neighbor, nearest_neighbor))
        }

        // let mut nearest_match = None;
        // let mut nearest_distance = f64::INFINITY;

        // // Mmm... tasty O(n^2)
        // for (i, a) in self.boxes.iter().enumerate() {
        //     for (j, b) in self.boxes.iter().enumerate() {
        //         if i == j {
        //             // No selfcest
        //             continue;
        //         }

        //         if self.hitchings.contains(&(i, j)) {
        //             continue
        //         }

        //         let distance = a.distance_to(b);
        //         if distance < nearest_distance {
        //             nearest_match = Some((i, j));
        //             nearest_distance = distance;
        //         }
        //     }
        // }

        // if let Some(m) = nearest_match {
        //     assert_eq!(self.nearest_neighbors.get(&m.0), Some(&m.1));
        //     assert_eq!(self.nearest_neighbors.get(&m.1), Some(&m.0));
        // }

        // nearest_match
    }

    fn hitch(&mut self, a: usize, b: usize) {
        // lazyyyyy...
        self.hitchings.insert((a, b));
        self.hitchings.insert((b, a));

        self.nearest_neighbor = None;

        self.compute_nearest_neighbors(a);
        self.compute_nearest_neighbors(b);

        let acirc = self.circuits_map.get(&a).cloned();
        let bcirc = self.circuits_map.get(&b).cloned();

        match (acirc, bcirc) {
            (None, None) => {
                self.circuits.insert(self.last_circuit, Circuit {
                    box_indices: HashSet::from([a, b]),
                });

                self.circuits_map.insert(a, self.last_circuit);
                self.circuits_map.insert(b, self.last_circuit);

                self.last_circuit += 1;
            },
            (None, Some(x)) => {
                self.circuits.get_mut(&x).unwrap().box_indices.insert(a);

                self.circuits_map.insert(a, x);
            },
            (Some(x), None) => {
                self.circuits.get_mut(&x).unwrap().box_indices.insert(b);

                self.circuits_map.insert(b, x);
            },
            (Some(x), Some(y)) if x != y => {
                let [Some(ac), Some(bc)] = self.circuits.get_disjoint_mut([&x, &y]) else {
                    panic!();
                };

                // eeny meeny miny moe... let's kill b's circuit
                for index in bc.box_indices.iter() {
                    // Reparent
                    self.circuits_map.insert(*index, x);
                    ac.box_indices.insert(*index);
                }

                self.circuits.remove(&y);
            },
            (Some(x), Some(y)) => {
                // Same-group hitching, just ignore...
                assert_eq!(x, y);
                assert!(!self.circuits.get_mut(&x).unwrap().box_indices.insert(a));
                assert!(!self.circuits.get_mut(&x).unwrap().box_indices.insert(b));
            },
        }
    }

    fn were_done(&self) -> bool {
        self.circuits.len() == 1 && self.circuits.values().next().unwrap().box_indices.len() == self.boxes.len()
    }

    fn get_final_bachs(&mut self) -> (usize, usize) {
        let final_bachs;

        loop {
            let bachs = self.closest_bachelors().expect("there's at least two singles like god damn get off my ass");

            self.hitch(bachs.0, bachs.1);

            if self.were_done() {
                final_bachs = bachs;
                break;
            }
        }

        final_bachs
    }

    fn circuitry_expenses(&self, largest: usize) -> usize {
        let mut circuits: Vec<_> = self.circuits.values().collect();

        circuits.sort_by_key(|c| std::cmp::Reverse(c.box_indices.len()));

        circuits.iter().take(largest).map(|circuit| circuit.box_indices.len()).for_each(|len| eprintln!("{len}"));

        circuits.iter().take(largest).map(|circuit| circuit.box_indices.len()).product()
    }
}

#[derive(PartialEq, Debug)]
struct Box {
    x: u64,
    y: u64,
    z: u64
}

impl Box {
    fn from_line(line: &str) -> Self {
        let mut splitz = line.split(',');
        let (Some(x), Some(y), Some(z)) = (splitz.next(), splitz.next(), splitz.next()) else {
            panic!("You must be at least a three dimensional being to enter this ride");
        };

        let (Ok(x), Ok(y), Ok(z)) = (x.parse(), y.parse(), z.parse()) else {
            panic!("???");
        };

        Box { x, y, z }
    }

    fn distance_to(&self, other: &Box) -> f64 {
        let f: f64 = (self.x.abs_diff(other.x).pow(2) + self.y.abs_diff(other.y).pow(2) + self.z.abs_diff(other.z).pow(2)) as f64;

        f.sqrt()
    }
}

#[derive(Debug)]
struct Circuit {
    box_indices: HashSet<usize>
}

fn part1() {
    let mut tour = WeebsLastTour::parse(INPUT);

    for _ in 0..1000 {
        let (a, b) = tour.closest_bachelors().unwrap();
        tour.hitch(a, b);
        eprintln!("{a} {b}");
    }

    dbg!(tour.circuitry_expenses(3));
}

fn part2() {
    let mut tour = WeebsLastTour::parse(INPUT);

    let final_bachs = tour.get_final_bachs();

    dbg!(tour.boxes[final_bachs.0].x * tour.boxes[final_bachs.1].x);
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

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn example() {
        let mut tour = WeebsLastTour::parse(EXAMPLE);
        let bachs = tour.closest_bachelors().expect("there's singles");

        assert_eq!(bachs, (0, 19));
        assert_eq!(tour.boxes[bachs.0], Box::from_line("162,817,812"));
        assert_eq!(tour.boxes[bachs.1], Box::from_line("425,690,689"));

        tour.hitch(bachs.0, bachs.1);
        // 2j + 18*1j

        let bachs = tour.closest_bachelors().expect("there's still singles");

        assert_eq!(bachs, (0, 7));
        assert_eq!(tour.boxes[bachs.0], Box::from_line("162,817,812"));
        assert_eq!(tour.boxes[bachs.1], Box::from_line("431,825,988"));

        tour.hitch(bachs.0, bachs.1);
        // 3j + 17*1j

        let bachs = tour.closest_bachelors().expect("there's still singles");

        assert_eq!(bachs, (2, 13));
        assert_eq!(tour.boxes[bachs.0], Box::from_line("906,360,560"));
        assert_eq!(tour.boxes[bachs.1], Box::from_line("805,96,715"));

        tour.hitch(bachs.0, bachs.1);

        // 3j + 2j + 15*1j

        // We hitched thrice, do the other 7 for 10 hitchings
        for i in 0..7 {
            let bachs = tour.closest_bachelors().expect("there's still singles");
            let (a, b) = (&tour.boxes[bachs.0], &tour.boxes[bachs.1]);

            eprintln!("{},{},{} <-> {}, {}, {} ({})", a.x, a.y, a.z, b.x, b.y, b.z, tour.circuits.len());

            tour.hitch(bachs.0, bachs.1);
        }

        dbg!(&tour.circuits);

        assert_eq!(tour.circuitry_expenses(3), 40);
    }

    #[test]
    fn example_part2() {
        let mut tour = WeebsLastTour::parse(EXAMPLE);

        let final_bachs = tour.get_final_bachs();

        assert_eq!(tour.boxes[final_bachs.0].x * tour.boxes[final_bachs.1].x, 25272);
    }
}
