use regex::Regex;
use z3::{Model, Solvable, Solver, ast::{Ast, Bool, Int}};

fn parse_input(input: String) -> (Vec<Vec<String>>, Vec<((u32, u32), Vec<u32>)>) {
    let mut shapes = Vec::new();
    let mut tests = Vec::new();
    let re_test = Regex::new(r"(\d+)x(\d+): ([0-9 ]+)").unwrap();
    let mut counter = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains('x') {
            let captures = re_test.captures(line).unwrap();
            let width = captures[1].parse::<u32>().unwrap();
            let height = captures[2].parse::<u32>().unwrap();
            let amount = captures[3].split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
            tests.push(((width, height), amount));
        } else if line.contains(':') {
            counter += 1;
        } else {
            if shapes.len() < counter {
                shapes.push(Vec::new());
            }
            shapes[counter - 1].push(line.to_string());
        }
    }
    (shapes, tests)
}

/*
#[derive(Clone)]
struct Shape {
    pub index: Int,
    pub x: Int,
    pub y: Int,
    pub flipped: Bool,
    pub rotation: Int,
}

impl Solvable for Shape {
    type ModelInstance = Self;
    fn read_from_model(&self, model: &Model, model_completion: bool) -> Option<Self> {
        Some(
            Self{
                index: model.eval(&self.index, model_completion).unwrap(),
                x: model.eval(&self.x, model_completion).unwrap(),
                y: model.eval(&self.y, model_completion).unwrap(),
                flipped: model.eval(&self.flipped, model_completion).unwrap(),
                rotation: model.eval(&self.rotation, model_completion).unwrap(),
            }
        )
    }

    fn generate_constraint(&self, model: &Self) -> Bool {
        Bool::or(&[
            self.index.eq(&model.index).not(),
            self.x.eq(&model.x).not(),
            self.y.eq(&model.y).not(),
            self.flipped.eq(&model.flipped).not(),
            self.rotation.eq(&model.rotation).not(),
        ])
    }
}

fn occupied_cells(shapes: &Vec<Vec<&str>>, shape: &Shape) -> Vec<(Int, Int)> {
    let mut cells = Vec::new();
    for i in 0..2 {
        for j in 0..2 {
            let new_i = i;
            let new_j = j;
            shape.rotation.
            Ast::
        }
    }
    cells
}

fn z3(shapes: &Vec<Vec<&str>>, test: &((u32, u32), Vec<u32>)) -> bool {
    let solver = Solver::new();
    let mut variables = Vec::new();
    let (width, height) = test.0;
    for i in 0..test.1.len() {
        for j in 0..test.1[i] {
            let shape = Shape {
                index: Int::from_i64(i as i64),
                x: Int::fresh_const("x"),
                y: Int::fresh_const("y"),
                flipped: Bool::fresh_const("flipped"),
                rotation: Int::fresh_const("rotation"),
            };
            solver.assert(shape.x.ge(0));
            solver.assert(shape.x.lt(width));
            solver.assert(shape.y.ge(0));
            solver.assert(shape.y.lt(height));
            solver.assert(shape.rotation.ge(0));
            solver.assert(shape.rotation.le(3));
            variables.push(shape);
        }
    }
    let mut cells = Vec::new();
    for shape in variables {
        cells.extend(occupied_cells(shapes, &shape));
    }
    solver.assert(Ast::distinct(&cells));
    match solver.check() {
        z3::SatResult::Sat => {
            return true
        },
        _ => return false
    }
}
*/

pub fn part1(input: String) -> u32 {
    let (shapes, tests) = parse_input(input);
    let mut count_crosses = Vec::new();
    for shape in shapes {
        let mut count = 0;
        for i in 0..=2 {
            for c in shape[i].chars() {
                if c == '#' {
                    count += 1;
                }
            }
        }
        count_crosses.push(count);
    }
    dbg!(&count_crosses);
    let mut ans = 0;
    for ((width, height), amount) in tests {
        let mut required = 0;
        for i in 0..amount.len() {
            required += count_crosses[i] * amount[i];
        }
        if required > width * height {
            println!("Impossible");
        } else if width / 3 * height / 3 >= amount.iter().sum() {
            ans += 1;
            println!("Totally possible");
        } else {
            println!("Think more");

        }
    }
    ans
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("0:
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
12x5: 1 0 1 0 3 2")), 2);
    }
}