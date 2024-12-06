use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(
        r"(?P<mul>mul\((?P<a>[0-9]+),(?P<b>[0-9]+)\))|(?P<do>do\(\))|(?P<dont>don't\(\))"
    )
    .unwrap();
}

fn solve(input: &str) -> u32 {
    let mut dont = false;
    MUL_REGEX
        .captures_iter(input)
        .map(|cap| {
            if let Some(_) = cap.name("mul") {
                let a: u32 = cap["a"].parse().unwrap();
                let b: u32 = cap["b"].parse().unwrap();
                if dont {
                    0
                } else {
                    a * b
                }
            } else if let Some(_) = cap.name("do") {
                dont = false;
                0
            } else if let Some(_) = cap.name("dont") {
                dont = true;
                0
            } else {
                unreachable!()
            }
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve(input), 48);
    }
}
