use array2d::Array2D;

fn parse_input(input: String) -> Array2D<char> {
    return Array2D::from_rows(
        &input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>()
    ).unwrap();
}

fn calc_neighbors((x, y): (usize, usize), data: &Array2D<char>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if 0 <= new_x && new_x < data.num_rows() as i32
                && 0 <= new_y && new_y < data.num_columns() as i32 {
                if data[(new_x as usize, new_y as usize)] == '@' {
                    neighbors.push((new_x as usize, new_y as usize));
                }
            }
        }
    }
    return neighbors;
}

pub fn part1(input: String) -> i32 {
    let mut ans = 0;
    let data = parse_input(input);
    for i in 0..data.num_rows() {
        for j in 0..data.num_columns() {
            if data[(i, j)] != '@' {
                continue;
            }
            if (calc_neighbors((i, j), &data).len() as i32) < 4 {
                ans += 1;
            }
        }
    }
    return ans;
}

pub fn part2(input: String) -> i32 {
    let mut total = 0;
    let mut data = parse_input(input);
    let mut new_data = data.clone();
    loop {
        let mut ans = 0;
        for i in 0..data.num_rows() {
            for j in 0..data.num_columns() {
                if data[(i, j)] != '@' {
                    continue;
                }
                if (calc_neighbors((i, j), &data).len() as i32) < 4 {
                    ans += 1;
                    new_data[(i, j)] = '.';
                }
            }
        }
        if ans > 0 {
            total += ans;
            data = new_data.clone();
        } else {
            break;
        }
    }
    return total;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.")), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.")), 43);
    }
}