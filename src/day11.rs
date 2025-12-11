use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn add_label(labels: &mut HashMap<String, u32>, label: &str, counter: &mut u32) {
    if !labels.contains_key(label) {
        labels.insert(label.to_string(), *counter);
        *counter += 1;
    }
}

fn parse_input(input: String) -> (HashMap<String, u32>, HashMap<u32, HashSet<u32>>, u32, u32) {
    let mut counter = 0;
    let mut labels = HashMap::new();
    let mut graph = HashMap::new();
    let re = Regex::new("([a-z]{3}):([ a-z]+)$").unwrap();
    add_label(&mut labels, "you", &mut counter);
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        add_label(&mut labels, &captures[1], &mut counter);
        for v in captures[2].split_ascii_whitespace() {
            add_label(&mut labels, v, &mut counter);
            let from = labels.get(&captures[1]).unwrap();
            let to = labels.get(v).unwrap();
            let edges = graph.entry(*from).or_insert_with(HashSet::new);
            edges.insert(*to);
        }
    }
    let start = *labels.get("you").unwrap();
    let end = *labels.get("out").unwrap();
    graph.insert(end, HashSet::new());
    (labels, graph, start, end)
}

fn dfs(v: u32, graph: &HashMap<u32, HashSet<u32>>, end: u32, paths: &mut Vec<i64>, visited: &mut Vec<bool>, exclude: &[u32]) -> i64 {
    if paths[v as usize] != 0 {
        return paths[v as usize];
    }
    if v == end {
        for e in exclude {
            if visited[*e as usize] {
                paths[v as usize] = -1;
                return 0;
            }
        }
        paths[v as usize] = 1;
        return 1;
    }
    let mut count_paths = 0;
    visited[v as usize] = true;
    for e in graph.get(&v).unwrap() {
        if !visited[*e as usize] {
            let res = dfs(*e, graph, end, paths, visited, exclude);
            if res == -1 {
                continue;
            }
            count_paths += res;
        }
    }
    visited[v as usize] = false;
    if count_paths == 0 {
        count_paths = -1;
    }
    paths[v as usize] = count_paths;
    count_paths
}

pub fn part1(input: String) -> i64 {
    let (labels, graph, start, end) = parse_input(input);
    let mut visited = vec![false; labels.len()];
    let mut paths = vec![0; labels.len()];
    dfs(start, &graph, end, &mut paths, &mut visited, &[])
}

pub fn part2(input: String) -> i64 {
    let (labels, graph, _, out) = parse_input(input);
    let start = *labels.get("svr").unwrap();
    let dac = *labels.get("dac").unwrap();
    let fft = *labels.get("fft").unwrap();
    
    let svr_dac = max(dfs(start, &graph, dac, &mut vec![0; labels.len()], &mut vec![false; labels.len()], &[fft]), 0);
    let dac_fft = max(dfs(dac, &graph, fft, &mut vec![0; labels.len()], &mut vec![false; labels.len()],&[]), 0);
    let fft_out = max(dfs(fft, &graph, out, &mut vec![0; labels.len()], &mut vec![false; labels.len()], &[dac]), 0);
    dbg!(svr_dac, dac_fft, fft_out);
    let path1 = svr_dac * dac_fft * fft_out;

    let svr_fft = max(dfs(start, &graph, fft, &mut vec![0; labels.len()], &mut vec![false; labels.len()], &[dac]), 0);
    let fft_dac = max(dfs(fft, &graph, dac, &mut vec![0; labels.len()], &mut vec![false; labels.len()], &[]), 0);
    let dac_out = max(dfs(dac, &graph, out, &mut vec![0; labels.len()], &mut vec![false; labels.len()], &[fft]), 0);
    dbg!(svr_fft, fft_dac, dac_out);
    let path2 = svr_fft * fft_dac * dac_out;
    path1 + path2
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out")), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(String::from("svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out")), 2);
    }
}