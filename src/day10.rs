use std::cmp::min;

use itertools::max;
use regex::Regex;

fn parse_input(input: String) -> Vec<(Vec<bool>, Vec<Vec<u32>>, Vec<u32>)> {
    let re_config = Regex::new(r"\[([#.]+)\]").unwrap();
    let re_buttons = Regex::new(r"\(([\d,]+)\)").unwrap();
    let re_joltage = Regex::new(r"\{([\d,]+)\}").unwrap();
    let mut machines = Vec::new();
    for line in input.lines() {
        let mut config = Vec::new();
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();
        for s in line.split_ascii_whitespace() {
            if s.starts_with('[') {
                config = re_config.captures(s).unwrap()[1].chars()
                    .map(|c| if c == '#' {true} else {false}).collect();
            } else if s.starts_with('{') {
                joltage = re_joltage.captures(s).unwrap()[1].split(',')
                    .map(|x| x.parse::<u32>().unwrap()).collect();
            } else {
                buttons.push(re_buttons.captures(s).unwrap()[1].split(',')
                    .map(|x| x.parse::<u32>().unwrap()).collect());
            }
        }
        machines.push((config, buttons, joltage));
    }
    machines
}

fn brute_force(cur_index: usize, pressed: &mut [bool], wanted: &[bool], cur_config: &mut [bool], buttons: &[Vec<u32>]) -> u32 {
    if cur_index == buttons.len() {
        if cur_config == wanted {
            return pressed.iter().filter(|b| **b).count() as u32;
        }
        return u32::MAX;
    }
    let option_1 = brute_force(cur_index + 1, pressed, wanted, cur_config, buttons);
    pressed[cur_index] = true;
    for button in &buttons[cur_index] {
        cur_config[*button as usize] = !cur_config[*button as usize];
    }
    let option_2 = brute_force(cur_index + 1, pressed, wanted, cur_config, buttons);
    pressed[cur_index] = false;
    for button in &buttons[cur_index] {
        cur_config[*button as usize] = !cur_config[*button as usize];
    }
    min(option_1, option_2)
}

pub fn part1(input: String) -> u32 {
    let machines = parse_input(input);
    let mut ans = 0;
    for (config, buttons, _) in machines {
        let mut pressed = vec![false; buttons.len()];
        let mut cur_config = vec![false; config.len()];
        ans += brute_force(0, &mut pressed, &config, &mut cur_config, &buttons);
    }
    ans
}

fn brute_force_joltage(cur_index: usize, presses: u32, cur_config: &mut [u32], buttons: &[Vec<u32>], max_joltage: u32) -> u32 {
    if cur_index == buttons.len() {
        if cur_config.iter().filter(|b| **b > 0).count() == 0 {
            return presses;
        }
        return u32::MAX;
    }
    let mut min_presses = brute_force_joltage(cur_index + 1, presses, cur_config, buttons, max_joltage);
    let mut pr_i = 0;
    'pr_i: while pr_i < max_joltage {
        for button in &buttons[cur_index] {
            if cur_config[*button as usize] == 0 {
                break 'pr_i;
            }
        }
        for button in &buttons[cur_index] {
            cur_config[*button as usize] -= 1;
        }
        pr_i += 1;
        let option = brute_force_joltage(
            cur_index + 1, 
            presses + pr_i,
            cur_config, 
            buttons,
            max_joltage,
        );
        min_presses = min(min_presses, option);
    }
    for button in &buttons[cur_index] {
        cur_config[*button as usize] += pr_i;
    }
    min_presses
}

pub fn part2(input: String) -> u32 {
    let machines = parse_input(input);
    let mut ans = 0;
    for (_, buttons, joltage) in machines {
        let max_joltage = joltage.iter().max().unwrap();
        let presses = brute_force_joltage(0, 0, &mut joltage.clone(), &buttons, *max_joltage);
        dbg!(presses);
        ans += presses;
    }
    ans
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")), 33);
    }
}