use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Problem {
    stone: u64,
    blinks: u8,
}

fn solve_problem(p: Problem, solved: &mut HashMap<Problem, u64>) -> u64 {
    if let Some(ans) = solved.get(&p) {
        return *ans;
    }

    if p.blinks == 0 {
        let ans = 1;
        solved.insert(p, ans);
        return ans;
    }

    let s = p.stone.to_string();
    if s.len() % 2 == 0 {
        let (a, b) = s.split_at(s.len() / 2);
        let a = Problem {
            stone: a.parse().unwrap(),
            blinks: p.blinks - 1,
        };
        let b = Problem {
            stone: b.parse().unwrap(),
            blinks: p.blinks - 1,
        };
        let ans = solve_problem(a, solved) + solve_problem(b, solved);
        solved.insert(p, ans);
        return ans;
    }

    let np = if p.stone == 0 {
        Problem {
            stone: 1,
            blinks: p.blinks - 1,
        }
    } else {
        Problem {
            stone: p.stone * 2024,
            blinks: p.blinks - 1,
        }
    };
    let ans = solve_problem(np, solved);
    solved.insert(p, ans);
    ans
}

fn solve(input: &str) -> u64 {
    let mut solved = HashMap::new();
    input
        .split_whitespace()
        .map(|s| {
            let stone: u64 = s.parse().unwrap();
            let problem = Problem { stone, blinks: 75 };
            solve_problem(problem, &mut solved)
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day11.txt");
    let ans = solve(input);
    println!("{}", ans);
}
