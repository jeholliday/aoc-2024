fn blink(stones: &[u64]) -> Vec<u64> {
    let mut new_stones = Vec::new();

    for stone in stones {
        let s = stone.to_string();
        if *stone == 0 {
            new_stones.push(1);
        } else if s.len() % 2 == 0 {
            let (a, b) = s.split_at(s.len() / 2);
            new_stones.push(a.parse().unwrap());
            new_stones.push(b.parse().unwrap());
        } else {
            new_stones.push(stone * 2024);
        }
    }

    new_stones
}

fn solve(input: &str) -> usize {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = blink(&stones);
    }

    stones.len()
}

fn main() {
    let input = include_str!("../../inputs/day11.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "125 17";
        assert_eq!(solve(input), 55312);
    }
}
