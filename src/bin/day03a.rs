use regex::Regex;

lazy_static::lazy_static!(
    static ref MUL_REGEX: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
);

fn solve(input: &str) -> u32 {
    MUL_REGEX
        .captures_iter(input)
        .map(|cap| {
            let a: u32 = cap[1].parse().unwrap();
            let b: u32 = cap[2].parse().unwrap();
            a * b
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day03.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve(input), 161);
    }
}
