#![allow(clippy::needless_range_loop)]

use std::{collections::{HashMap, HashSet}, fmt::{Display, Write as _}, fs::OpenOptions, io::Write as _, sync::Mutex};
use good_lp::{
    Expression, Solution as _, SolverModel, Variable, constraint, default_solver, solvers::highs::HighsSolution, variable, variables
};
use rayon::{ThreadPoolBuilder, iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator}};

use mimalloc::MiMalloc;

// Switching to a more efficient allocator makes a small difference in solver speed
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[allow(unused)]
const INPUT: &str = include_str!("../inputs/day12.txt");

#[derive(Debug)]
struct BullshitPacking {
    shapes: Vec<Shape>,
    shape_variants: Vec<ShapeVariant>,
    regions: Vec<Region>
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    required_presents: Vec<usize>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    width: usize,
    height: usize,
    rows: Vec<Vec<bool>>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ShapeVariant {
    derived_index: usize,
    width: usize,
    height: usize,
    rows: Vec<Vec<bool>>
}

struct Placement {
    shape_id: usize,
    variant_id: usize,
    start_x: usize,
    start_y: usize,
    covered_cells: Vec<usize>,
    var: Option<Variable>
}

impl Display for ShapeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_char(if self.rows[y][x] { '#' } else { '.' })?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Shape {
    fn from_lines<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Self {
        let mut rows = vec![];
        let mut width = 0;
        let mut height = 0;

        while let Some(line) = lines.next() && !line.is_empty() {
            let cells: Vec<_> = line.chars().map(|c| c == '#').collect();

            width = cells.len();
            height += 1;

            rows.push(cells);
        }

        Self {
            width,
            height,
            rows
        }
    }

    fn rotated_90deg(&self) -> Shape {
        let mut new_rows = vec![vec![false; self.height]; self.width];

        for y in 0..self.height {
            for x in 0..self.width {
                new_rows[x][self.height - 1 - y] = self.rows[y][x];
            }
        }

        Shape {
            width: self.height,
            height: self.width,
            rows: new_rows,
        }
    }

    // Flipping horizontally (or vertically) is all we need for rotating square shapes
    fn flipped(&self) -> Shape {
        let mut new_rows = vec![vec![false; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                new_rows[y][self.width - 1 - x] = self.rows[y][x];
            }
        }

        Shape {
            width: self.width,
            height: self.height,
            rows: new_rows,
        }
    }

    fn variants(&self, shape_index: usize) -> Vec<ShapeVariant> {
        let mut set = HashSet::new();
        let mut list = vec![];
        let mut shape = self.clone();

        for _ in 0..4 {
            let next = shape.rotated_90deg();
            let flipped = shape.flipped();

            let a = ShapeVariant { derived_index: shape_index, width: shape.width, height: shape.height, rows: shape.rows };
            let b = ShapeVariant { derived_index: shape_index, width: flipped.width, height: flipped.height, rows: flipped.rows };

            // eprintln!("rotated:\n{a}");
            // eprintln!("flipped:\n{b}");

            // Filter duplicates while retaining order
            if set.insert(a.clone()) {
                list.push(a);
            }

            if set.insert(b.clone()) {
                list.push(b);
            }

            shape = next;
        }

        list
    }

    fn cell_count(&self) -> usize {
        self.rows.iter().fold(0, |sum, row| sum + row.iter().filter(|c| **c).count())
    }
}

impl BullshitPacking {
    fn parse(s: &str) -> Self {
        let mut shapes = vec![];
        let mut regions = vec![];
        let mut lines = s.lines().peekable();

        while let Some(line) = lines.peek() && line.trim_start_matches(|c: char| c.is_ascii_digit() || c == ':').is_empty() {
            lines.next(); // assert line = shapes.len():

            shapes.push(Shape::from_lines(&mut lines));
        }

        // The regions and required presents
        for line in lines {
            let removed = line.trim_start_matches(|c: char| c.is_ascii_digit());
            let width = line[0..line.len() - removed.len()].parse().unwrap();
            let line = removed.trim_start_matches('x');
            let removed = line.trim_start_matches(|c: char| c.is_ascii_digit());
            let height = line[0..line.len() - removed.len()].parse().unwrap();
            let line = removed.strip_prefix(": ").unwrap();
            let required_presents = line.split(' ').map(|p| p.parse().unwrap()).collect();

            regions.push(Region {
                width,
                height,
                required_presents
            });
        }

        let shape_variants = shapes.iter().enumerate().flat_map(|(index, shape)| shape.variants(index)).collect();

        Self {
            shapes,
            shape_variants,
            regions
        }
    }
}

impl Region {
    // I'm gonna guess that brute forcing won't work
    // First idea: Extend the weird shapes into larger, rectangle-shaped objects that might fit
    // Then pass it to a rectangle solver like rectpack or crunch
    // Not definitely correct and doesn't solve the problem of exact cover without missing some cases
    // Can be tried repeatedly with different rectangle combinations but I'm still not sure it'll fit everything
    // And verifying stragglers might be hard
    //
    // Second idea: I just used a linear programming solver with integer variables for day 10
    // No more dependencies, and each cell could be represented as a 0-1 integer variable
    // with a constraint on being 0 or 1

    fn might_fit_trivially(&self, shapes: &[Shape]) -> bool {
        let needed_shapes: Vec<_> = self.required_presents.iter().enumerate().flat_map(|(index, count)| {
            (0..*count).map(move |_| shapes[index].clone())
        }).collect();

        if needed_shapes.iter().fold(0, |sum, shape| sum + shape.cell_count()) > self.width * self.height {
            return false;
        }

        true
    }

    #[allow(unused)]
    fn can_fit_shapes(&self, shapes: &[Shape]) -> bool {
        let mut needed_shapes: Vec<_> = self.required_presents.iter().enumerate().flat_map(|(index, count)| {
            (0..*count).map(move |_| shapes[index].clone())
        }).collect();

        // It was worth a shot
        if needed_shapes.iter().fold(0, |sum, shape| sum + shape.cell_count()) > self.width * self.height {
            // Wtf it actually saved time
            eprintln!("early return; you shouldn't see this if you filter by might_fit_trivially");

            return false;
        }

        fn generate_placements(
            shape_id: usize,
            variants: &[ShapeVariant],
            region_w: usize,
            region_h: usize,
        ) -> Vec<Placement> {
            let mut placements = Vec::new();

            for (variant_id, v) in variants.iter().enumerate() {
                for y in 0..=region_h - v.height {
                    for x in 0..=region_w - v.width {
                        let mut cells = Vec::new();
                        let mut valid = true;

                        for dy in 0..v.height {
                            for dx in 0..v.width {
                                if v.rows[dy][dx] {
                                    let cx = x + dx;
                                    let cy = y + dy;
                                    cells.push(cy * region_w + cx);
                                }
                            }
                        }

                        if valid {
                            placements.push(Placement {
                                shape_id,
                                variant_id,
                                start_x: x,
                                start_y: y,
                                covered_cells: cells,
                                var: None,
                            });
                        }
                    }
                }
            }

            placements
        }

        let mut vars = variables!();
        let mut placements = vec![];

        for (index, shape) in shapes.iter().enumerate() {
            let variants = shape.variants(index);

            placements.extend(generate_placements(index, &variants, self.width, self.height));
        }

        for p in placements.iter_mut() {
            p.var = Some(vars.add(variable().integer().min(0).max(1)));
        }

        // Flatten cells into a linear array
        let mut cell_to_vars = vec![Vec::<Variable>::new(); self.width * self.height];

        for p in &placements {
            for &cell in &p.covered_cells {
                cell_to_vars[cell].push(p.var.unwrap());
            }
        }

        let mut model = vars.minimise(0).using(default_solver);

        for vars in cell_to_vars {
            if !vars.is_empty() {
                model = model.with(constraint!(
                    vars.iter().sum::<Expression>() <= 1
                ));
            }
        }

        let mut shape_to_vars = vec![Vec::<Variable>::new(); shapes.len()];

        for p in &placements {
            shape_to_vars[p.shape_id].push(p.var.unwrap());
        }

        for (index, vars) in shape_to_vars.iter().enumerate() {
            let required = self.required_presents[index] as f64;

            model = model.with(vars.iter().sum::<Expression>().eq(self.required_presents[index] as f64));
        }

        let solution = model.solve();

        if let Ok(solution) = &solution {
            for p in &placements {
                #[allow(clippy::collapsible_if)]
                if solution.value(p.var.unwrap()) > 0.5 {
                    if solution.value(p.var.unwrap()) > 1.5 {
                        eprintln!("Greater than 1.0 {}", solution.value(p.var.unwrap()));
                    }

                    // println!(
                    //     "Shape {} variant {} at ({}, {})",
                    //     p.shape_id, p.variant_id, p.start_x, p.start_y
                    // );
                }
            }

            let verified = self.verify_solution(shapes, &placements, solution);
            if !verified {
                eprintln!("Did not pass verification");
                return false;
            }
        }

        solution.is_ok()
    }

    fn verify_solution(&self, shapes: &[Shape], placements: &[Placement], solution: &HighsSolution) -> bool {
        let needed_cells_for_shapes = self.cells_needed(shapes);

        let mut rows = vec![vec![0usize; self.width]; self.height];

        let mut shape_variants = vec![vec![]; shapes.len()];

        for (shape_index, shape) in shapes.iter().enumerate() {
            shape_variants[shape_index] = shape.variants(shape_index);
        }

        for placement in placements {
            if solution.value(placement.var.unwrap()) < 0.5 {
                continue;
            }

            let variant = &shape_variants[placement.shape_id][placement.variant_id];

            for y in 0..variant.rows.len() {
                for x in 0..variant.rows[y].len() {
                    if variant.rows[y][x] {
                        rows[placement.start_y + y][placement.start_x + x] += 1;
                    }
                }
            }
        }

        let mut buf = String::new();
        let mut verified = true;
        let mut cell_count = 0;

        for (y, row) in rows.iter().enumerate() {
            for (x, count) in row.iter().cloned().enumerate() {
                write!(&mut buf, "{count}").unwrap();

                cell_count += count;

                if count > 1 {
                    verified = false;
                }
            }

            writeln!(&mut buf).unwrap();
        }

        eprintln!("{buf}\ncells filled: {cell_count} cells needed: {needed_cells_for_shapes} rect size: {}", self.rect_size());

        verified
    }

    fn cells_needed(&self, shapes: &[Shape]) -> usize {
        let needed_shapes: Vec<_> = self.required_presents.iter().enumerate().flat_map(|(index, count)| {
            (0..*count).map(move |_| shapes[index].clone())
        }).collect();

        needed_shapes.iter().fold(0, |sum, shape| sum + shape.cell_count())
    }

    fn rect_size(&self) -> usize {
        self.width * self.height
    }
}

fn part1() {
    let mut packing = BullshitPacking::parse(INPUT);

    // dbg!(packing.shapes.len());
    // dbg!(packing.shape_variants.len());

    // for variant in packing.shape_variants {
    //     // eprintln!("{}:\n{}", variant.derived_index, variant);
    // }

    // Each solver can take up to 3-4gb, and I don't have much more than 20gb to spare for this
    // I should've bought 64gb of ram
    ThreadPoolBuilder::new().num_threads(10).build_global().unwrap();

    let mut non_trivial_results: HashMap<usize, bool> = HashMap::new();

    // I've yet to see a non-trivial result churn out false
    include_str!("day12cache.txt").lines().for_each(|line| {
        let (index, fits) = line.split_once(": ").unwrap();
        let (index, fits) = (index.parse().unwrap(), fits == "true");

        non_trivial_results.insert(index, fits);
    });

    eprintln!("regions: {}", packing.regions.len());
    eprintln!("non-trivial regions that might fit (upper ceiling): {}", packing.regions.iter().filter(|region| region.might_fit_trivially(&packing.shapes)).count());
    eprintln!("regions that trivially don't fit: {}", packing.regions.iter().enumerate().filter(|(_, region)| !region.might_fit_trivially(&packing.shapes)).map(|(i, _)| i.to_string()).collect::<Vec<_>>().join(","));


    let cache = OpenOptions::new()
        .append(true)
        .open("src/bin/day12cache.txt")
        .unwrap();
    let cache = Mutex::new(cache);

    // Sort by tightest fits
    packing.regions.sort_by_key(|region| region.rect_size() as i64 - region.cells_needed(&packing.shapes) as i64);

    eprintln!("{}", packing.regions.first().unwrap().rect_size() as i64 - packing.regions.first().unwrap().cells_needed(&packing.shapes) as i64);
    eprintln!("{}", packing.regions.last().unwrap().rect_size() as i64 - packing.regions.last().unwrap().cells_needed(&packing.shapes) as i64);

    packing.regions.par_iter().enumerate()
        .filter(|(_, region)| region.might_fit_trivially(&packing.shapes))
        .for_each(|(index, region)| {
            let (cached, fits) = if non_trivial_results.contains_key(&index) {
                (true, non_trivial_results.get(&index).cloned().unwrap())
            } else {
                (false, region.can_fit_shapes(&packing.shapes))
            };

            if !cached {
                eprintln!("{index} fits: {fits}");

                let mut cache = cache.lock().unwrap();
                writeln!(&mut cache, "{index}: {fits}").unwrap();
            } else {
                eprintln!("{index} fits: {fits} (cached)");
            }
        });

    // Fuck you Eric
}

fn part2() {
    fn parse_placements(s: &str) -> Vec<Placement> {
        s.lines().map(|line| {
            let line = line.strip_prefix("Shape ").unwrap();
            let (shape_index, line) = (line[0..1].parse().unwrap(), &line[1..]);
            let line = line.strip_prefix(" variant ").unwrap();
            let (variant_index, line) = (line[0..1].parse().unwrap(), &line[1..]);
            let line = line.strip_prefix(" at (").unwrap();
            let removed = line.trim_start_matches(|c: char| c.is_ascii_digit());
            let x = line[0..line.len() - removed.len()].parse().unwrap();
            let line = removed.strip_prefix(", ").unwrap();
            let removed = line.trim_start_matches(|c: char| c.is_ascii_digit());
            let y = line[0..line.len() - removed.len()].parse().unwrap();

            Placement {
                shape_id: shape_index,
                variant_id: variant_index,
                start_x: x,
                start_y: y,
                covered_cells: vec![],
                var: None,
            }
        }).collect()
    }

    let packing = BullshitPacking::parse(INPUT);

    let parsed_placements = parse_placements("Shape 0 variant 0 at (1, 2)
Shape 0 variant 0 at (3, 17)
Shape 0 variant 0 at (2, 25)
Shape 0 variant 0 at (24, 28)
Shape 0 variant 0 at (16, 32)
Shape 0 variant 0 at (29, 32)
Shape 0 variant 1 at (9, 22)
Shape 0 variant 1 at (3, 23)
Shape 0 variant 1 at (0, 26)
Shape 0 variant 1 at (27, 29)
Shape 0 variant 2 at (13, 0)
Shape 0 variant 2 at (6, 12)
Shape 0 variant 2 at (28, 26)
Shape 0 variant 3 at (23, 15)
Shape 0 variant 3 at (12, 18)
Shape 0 variant 3 at (6, 24)
Shape 0 variant 4 at (32, 22)
Shape 0 variant 5 at (6, 9)
Shape 0 variant 5 at (30, 10)
Shape 0 variant 5 at (12, 12)
Shape 0 variant 5 at (0, 20)
Shape 0 variant 6 at (32, 0)
Shape 0 variant 6 at (5, 3)
Shape 0 variant 6 at (3, 10)
Shape 0 variant 6 at (32, 14)
Shape 0 variant 6 at (14, 17)
Shape 0 variant 6 at (6, 21)
Shape 1 variant 0 at (21, 0)
Shape 1 variant 0 at (18, 8)
Shape 1 variant 0 at (26, 9)
Shape 1 variant 0 at (1, 11)
Shape 1 variant 0 at (5, 27)
Shape 1 variant 0 at (31, 29)
Shape 1 variant 0 at (13, 34)
Shape 1 variant 1 at (0, 5)
Shape 1 variant 1 at (29, 7)
Shape 1 variant 1 at (10, 16)
Shape 1 variant 2 at (6, 0)
Shape 1 variant 2 at (19, 4)
Shape 1 variant 3 at (20, 1)
Shape 1 variant 3 at (22, 5)
Shape 1 variant 3 at (9, 6)
Shape 1 variant 3 at (32, 7)
Shape 1 variant 3 at (15, 9)
Shape 1 variant 3 at (19, 9)
Shape 1 variant 3 at (30, 30)
Shape 2 variant 0 at (16, 0)
Shape 2 variant 0 at (12, 6)
Shape 2 variant 0 at (25, 6)
Shape 2 variant 0 at (26, 15)
Shape 2 variant 0 at (17, 18)
Shape 2 variant 0 at (12, 23)
Shape 2 variant 0 at (29, 28)
Shape 2 variant 0 at (14, 29)
Shape 2 variant 1 at (18, 13)
Shape 2 variant 1 at (0, 14)
Shape 2 variant 1 at (32, 17)
Shape 2 variant 2 at (10, 0)
Shape 2 variant 2 at (22, 2)
Shape 2 variant 2 at (13, 3)
Shape 2 variant 2 at (26, 6)
Shape 2 variant 2 at (32, 11)
Shape 2 variant 2 at (26, 12)
Shape 2 variant 2 at (17, 15)
Shape 2 variant 2 at (25, 17)
Shape 2 variant 2 at (8, 25)
Shape 2 variant 2 at (25, 26)
Shape 2 variant 2 at (13, 31)
Shape 2 variant 3 at (25, 3)
Shape 2 variant 3 at (17, 5)
Shape 2 variant 3 at (29, 12)
Shape 2 variant 3 at (6, 18)
Shape 2 variant 3 at (17, 21)
Shape 2 variant 3 at (15, 26)
Shape 2 variant 3 at (3, 29)
Shape 3 variant 0 at (3, 0)
Shape 3 variant 0 at (0, 16)
Shape 3 variant 0 at (26, 20)
Shape 3 variant 0 at (19, 26)
Shape 3 variant 0 at (0, 29)
Shape 3 variant 0 at (26, 32)
Shape 3 variant 0 at (10, 33)
Shape 3 variant 1 at (17, 24)
Shape 3 variant 1 at (11, 26)
Shape 3 variant 2 at (23, 8)
Shape 3 variant 3 at (17, 1)
Shape 3 variant 3 at (29, 2)
Shape 3 variant 3 at (3, 13)
Shape 3 variant 3 at (28, 18)
Shape 3 variant 3 at (18, 19)
Shape 3 variant 3 at (21, 28)
Shape 3 variant 4 at (22, 10)
Shape 3 variant 4 at (31, 19)
Shape 3 variant 4 at (0, 23)
Shape 3 variant 5 at (27, 23)
Shape 3 variant 5 at (7, 30)
Shape 3 variant 5 at (16, 30)
Shape 3 variant 6 at (9, 13)
Shape 3 variant 7 at (6, 6)
Shape 3 variant 7 at (9, 10)
Shape 3 variant 7 at (9, 19)
Shape 3 variant 7 at (2, 20)
Shape 3 variant 7 at (14, 20)
Shape 4 variant 0 at (32, 3)
Shape 4 variant 0 at (0, 8)
Shape 4 variant 0 at (11, 9)
Shape 4 variant 0 at (23, 18)
Shape 4 variant 0 at (21, 19)
Shape 4 variant 0 at (24, 21)
Shape 4 variant 0 at (21, 22)
Shape 4 variant 0 at (24, 24)
Shape 4 variant 0 at (22, 25)
Shape 4 variant 0 at (17, 27)
Shape 4 variant 0 at (11, 30)
Shape 4 variant 0 at (23, 31)
Shape 4 variant 0 at (5, 32)
Shape 4 variant 0 at (19, 32)
Shape 4 variant 0 at (7, 33)
Shape 4 variant 0 at (22, 34)
Shape 4 variant 0 at (32, 34)
Shape 4 variant 1 at (9, 3)
Shape 4 variant 1 at (16, 3)
Shape 4 variant 1 at (14, 7)
Shape 4 variant 1 at (15, 14)
Shape 4 variant 1 at (21, 14)
Shape 4 variant 1 at (20, 16)
Shape 4 variant 1 at (14, 24)
Shape 4 variant 1 at (8, 28)
Shape 5 variant 0 at (9, 0)
Shape 5 variant 0 at (27, 0)
Shape 5 variant 0 at (29, 0)
Shape 5 variant 0 at (13, 2)
Shape 5 variant 0 at (8, 9)
Shape 5 variant 0 at (32, 10)
Shape 5 variant 0 at (14, 12)
Shape 5 variant 0 at (16, 12)
Shape 5 variant 0 at (6, 14)
Shape 5 variant 0 at (28, 16)
Shape 5 variant 0 at (11, 21)
Shape 5 variant 0 at (32, 26)
Shape 5 variant 1 at (0, 0)
Shape 5 variant 1 at (25, 2)
Shape 5 variant 1 at (3, 6)
Shape 5 variant 1 at (20, 7)
Shape 5 variant 1 at (20, 12)
Shape 5 variant 1 at (7, 16)
Shape 5 variant 1 at (6, 17)
Shape 5 variant 1 at (4, 21)
Shape 5 variant 1 at (1, 32)
Shape 5 variant 1 at (0, 33)
Shape 5 variant 2 at (4, 4)
Shape 5 variant 2 at (28, 4)
Shape 5 variant 2 at (30, 4)
Shape 5 variant 2 at (3, 7)
Shape 5 variant 2 at (29, 20)
Shape 5 variant 2 at (19, 21)
Shape 5 variant 2 at (30, 23)
Shape 5 variant 2 at (20, 24)
Shape 5 variant 3 at (23, 11)
Shape 5 variant 3 at (24, 12)
Shape 5 variant 3 at (12, 15)
Shape 5 variant 3 at (30, 15)
Shape 5 variant 3 at (32, 20)
Shape 5 variant 3 at (12, 27)");

    // dbg!(packing.regions[515].verify_solution(&packing.shapes, &parsed_placements));
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

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn example() {
        let packing = BullshitPacking::parse(EXAMPLE);

        dbg!(packing.regions[0].can_fit_shapes(&packing.shapes));
        dbg!(packing.regions[1].can_fit_shapes(&packing.shapes));
        dbg!(packing.regions[2].can_fit_shapes(&packing.shapes));

        todo!();
    }

    #[test]
    fn example_part2() {
        // todo!();
    }
}
