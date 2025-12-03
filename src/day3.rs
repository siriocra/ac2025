use array2d::Array2D;

fn parse_input(input: String) -> Array2D<u32> {
    return Array2D::from_rows(
        &input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()
    ).unwrap();
}

pub fn part1(input: String) -> u32 {
    let data = parse_input(input);
    let mut max_num = 0;
    let mut ans = 0;
    for i in 0..data.num_rows() {
        max_num = 0;
        for j in 0..data.num_columns() {
            for j2 in j+1..data.num_columns() {
                let new_num = data[(i, j)] * 10 + data[(i, j2)];
                if new_num > max_num {
                    max_num = new_num;
                }
                
            }
        }
        ans += max_num;
    }
    return ans;
}

pub fn part2(input: String) -> u64 {
    let data = parse_input(input);
    let len = 12;
    let mut ans: u64 = 0;
    for i in 0..data.num_rows() {
        let mut max_num = ((data.num_columns() - len)..data.num_columns()).collect::<Vec<_>>();
        for j in (0..(data.num_columns() - len)).rev() {
            if data[(i, j)] >= data[(i, max_num[0])] {
                let mut new_index = j;
                let mut counter = 0;
                while counter < len && data[(i, new_index)] >= data[(i, max_num[counter])] {
                    let old_index = max_num[counter];
                    max_num[counter] = new_index;
                    counter += 1;
                    new_index = old_index;
                }
            }
        }
        let mut num: u64 = 0;
        for j in 0..len {
            num = num * 10 + data[(i, max_num[j])] as u64;
        }
        println!("{} {}", i, num);
        ans += num;
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("987654321111111
811111111111119
234234234234278
818181911112111")), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("987654321111111
811111111111119
234234234234278
818181911112111")), 3121910778619);
    }
}