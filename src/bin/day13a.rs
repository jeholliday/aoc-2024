use regex::Regex;

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"Button A: X\+([0-9]+), Y\+([0-9]+)\s+Button B: X\+([0-9]+), Y\+([0-9]+)\s+Prize: X=([0-9]+), Y=([0-9]+)"
    )
    .unwrap();
}

struct Problem {
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
    x: usize,
    y: usize,
}

fn solve_problem(p: &Problem) -> Option<usize> {
    let mut solutions: Vec<usize> = Vec::new();
    for a in 0..=100 {
        for b in 0..=100 {
            let x = a * p.ax + b * p.bx;
            let y = a * p.ay + b * p.by;
            if x == p.x && y == p.y {
                let tokens = a * 3 + b;
                solutions.push(tokens);
            }
        }
    }
    solutions.iter().min().cloned()
}

fn solve(input: &str) -> usize {
    let problems: Vec<Problem> = REGEX
        .captures_iter(input)
        .map(|c| {
            let (_, [ax, ay, bx, by, x, y]) = c.extract();
            Problem {
                ax: ax.parse().unwrap(),
                ay: ay.parse().unwrap(),
                bx: bx.parse().unwrap(),
                by: by.parse().unwrap(),
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();
    problems.iter().filter_map(solve_problem).sum()
}

fn main() {
    let input = include_str!("../../inputs/day13.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279";
        assert_eq!(solve(input), 480);
    }
}
