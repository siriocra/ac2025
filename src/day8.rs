use std::cmp::Reverse;

use array2d::Array2D;
use dsu_tree::DsuRoot;

fn parse_input(input: String) -> Vec<Vec<i64>> {
    let mut coords = Vec::new();
    for line in input.lines() {
        coords.push(line.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>());
    }
    coords
}

fn dist(p1: &[i64], p2: &[i64]) -> i64 {
    let mut distance: i64 = 0;
    for x in 0..p1.len() {
        distance += (p2[x] - p1[x]) * (p2[x] - p1[x]);
    }
    distance
}

fn calc_distances(coords: &[Vec<i64>]) -> Array2D<i64> {
    let mut distances = Array2D::filled_with(0, coords.len(), coords.len());
    for i in 0..coords.len() {
        for j in 0..coords.len() {
            distances[(i, j)] = dist(&coords[i], &coords[j]);
        }
    }
    distances
}

fn calc_circiuts(coords: &Vec<Vec<i64>>, turns: i32) -> (Vec<i32>, usize, usize) {
    let mut distances = calc_distances(coords);
    let mut roots: Vec<DsuRoot<_>> = Vec::new();
    for i in 0..coords.len() {
        roots.push(DsuRoot::new(i+1));
    }
    let mut min_i = 0;
    let mut min_j = 0;
    'turns: for _ in 0..turns {
        let mut min_distance = i64::MAX;
        for i in 0..coords.len() {
            for j in i+1..coords.len() {
                if i == j {
                    continue;
                }
                if min_distance > distances[(i, j)] {
                    min_distance = distances[(i, j)];
                    min_i = i;
                    min_j = j;
                }
            }
        }
        if min_distance == i64::MAX {
            break;
        }
        distances[(min_i, min_j)] = i64::MAX;
        distances[(min_j, min_i)] = i64::MAX;
        let (dsu_1, dsu_2) = roots.split_at_mut(min_j);
        dsu_1[min_i].merge_into(&mut dsu_2[0]);
        for j in 0..dsu_1.len() {
            if !DsuRoot::same(&mut dsu_2[0], &mut dsu_1[j]) {
                continue 'turns;
            }
        }
        for j in 1..dsu_2.len() {
            if !DsuRoot::same(&mut dsu_1[min_i], &mut dsu_2[j]) {
                continue 'turns;
            }
        }
        break;
    }
    let mut circuits = Vec::new();
    let mut visited = vec![false; roots.len()];
    for i in 0..roots.len() {
        visited[i] = true;
        let mut circuit = 1;
        for j in i+1..roots.len() {
            if visited[j] {
                continue;
            }
            let (dsu_1, dsu_2) = roots.split_at_mut(j);
            if DsuRoot::same(&mut dsu_1[i], &mut dsu_2[0]) {
                circuit += 1;
                visited[j] = true;
            }
        }
        circuits.push(circuit);
    }
    for circuit in &circuits {
        print!(" {circuit} ");
    }
    println!();
    circuits.sort_unstable_by_key(|x| Reverse(*x));
    (circuits, min_i, min_j)
}

pub fn part1(input: String) -> i32 {
    let coords = parse_input(input);
    let (circuits, _, _) = calc_circiuts(&coords, 1000);
    circuits[0] * circuits[1] * circuits[2]
}

pub fn part2(input: String) -> i64 {
    let coords = parse_input(input);
    let (_, min_i, min_j) = calc_circiuts(&coords, coords.len() as i32 * coords.len() as i32);
    coords[min_i][0] * coords[min_j][0]
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (circuits, _, _) = calc_circiuts(&parse_input(String::from("162,817,812
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
425,690,689")), 10);
        assert_eq!(circuits[0] * circuits[1] * circuits[2], 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("162,817,812
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
425,690,689")), 25272);
    }
}