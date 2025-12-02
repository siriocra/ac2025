use std::collections::HashMap;

use regex::Regex;

fn parse_input(input: String) -> Vec<(i64, i64)> {
    let mut ranges = Vec::new();
    let r = input.split(',').collect::<Vec<_>>();
    for range in r {
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        let captures = re.captures(&range).unwrap();
        ranges.push((
            captures[1].parse::<i64>().unwrap(),
            captures[2].parse::<i64>().unwrap(),
        ));
    }
    return ranges;
}

fn round(len: usize) -> i64 {
    return 10i64.pow((len - 1) as u32);
}

fn calc(min_num: i64, max_num: i64, repeats: usize, cache: &mut HashMap<i64, bool>) -> i64 {
    let mut ans = 0;
    let min_num_str = min_num.to_string();
    let part_len = min_num_str.len() / repeats;
    let (first, _) = min_num_str.split_at(part_len);
    let mut part = first.to_string();
    loop {
        let mut full_str = part.clone();
        for i in 1..repeats {
            full_str += &part;
        } 
        let full = full_str.parse::<i64>().unwrap();
        if full > max_num {
            break;
        }
        if full >= min_num && full <= max_num {
            println!("full {} {}", full, repeats);
            if !cache.contains_key(&full) { 
                ans += full;
                cache.insert(full, true);
            }
        }
        part = (part.parse::<i64>().unwrap() + 1).to_string();
    }
    return ans;
}

pub fn part1(input: String) -> i64 {
    let ranges = parse_input(input);
    let mut ans: i64 = 0;
    let mut cache = HashMap::new();
    for (min_v, max_v) in ranges {
        let (min_v_len, max_v_len) = (min_v.to_string().len(), max_v.to_string().len());
        if min_v_len % 2 == 1 {
            if min_v_len == max_v_len {
                // No invalid ids
                continue;
            } else {
                ans += calc(round(min_v_len + 1), max_v, 2, &mut cache);
            }
        } else {
            ans += calc(min_v, max_v, 2, &mut cache);
        }
    }
    return ans;
}

pub fn part2(input: String) -> i64 {
    let ranges = parse_input(input);
    let mut ans: i64 = 0;
    for (min_v, max_v) in ranges {
        let (min_v_len, max_v_len) = (min_v.to_string().len(), max_v.to_string().len());
        let mut cache = HashMap::new();
        for repeats in [2, 3, 5, 7] {
            if min_v_len % repeats != 0 {
                if min_v_len == max_v_len {
                    // No invalid ids
                    continue;
                } else {
                    let correct_len = min_v_len + repeats - min_v_len % repeats;
                    ans += calc(round(correct_len), max_v, repeats, &mut cache);
                }
            } else {
                ans += calc(min_v, max_v, repeats, &mut cache);
            }
        }
    }
    return ans;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124")), 1227775554);
    }

    #[test]
    fn test_round() {
        assert_eq!(round(2), 10);
        assert_eq!(round(3), 100);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124")), 4174379265);
    }
}