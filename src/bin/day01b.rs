use std::collections::HashMap;

use itertools::Itertools;

fn solve(input: &str) -> u32 {
    let input: Vec<&str> = input.trim().split("\n").map(str::trim).collect();
    let pairs: Vec<(u32, u32)> = input
        .iter()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();
    let a: Vec<u32> = pairs.iter().map(|(a, _)| a).copied().sorted().collect();
    let b: Vec<u32> = pairs.iter().map(|(_, b)| b).copied().sorted().collect();

    let b_counts = b.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    a.iter().map(|a| a * b_counts.get(a).unwrap_or(&0)).sum()
}

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        assert_eq!(solve(input), 31);
    }
}
