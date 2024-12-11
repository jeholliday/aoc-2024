use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

#[inline]
fn num_digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

fn apply_operations(value: u64, numbers: &Vec<u64>, operations: &Vec<Operation>) -> bool {
    let mut cur_val = numbers[0];
    for (i, number) in numbers.iter().skip(1).enumerate() {
        match operations[i] {
            Operation::Add => cur_val += number,
            Operation::Multiply => cur_val *= number,
            Operation::Concatenate => {
                cur_val = {
                    let digits = num_digits(*number);
                    cur_val * 10u64.pow(digits) + number
                }
            }
        }
        if cur_val > value {
            return false;
        }
    }
    cur_val == value
}

fn is_possible(value: u64, numbers: &Vec<u64>) -> bool {
    let num_operations = numbers.len() - 1;
    let variants = vec![Operation::Add, Operation::Multiply, Operation::Concatenate];

    // Test all possible combinations of operations to see if the value is generated
    for operations in std::iter::repeat(variants)
        .take(num_operations)
        .multi_cartesian_product()
    {
        if apply_operations(value, numbers, &operations) {
            return true;
        }
    }
    false
}

fn solve(input: &str) -> u64 {
    // Parse input into tuples of (value, numbers)
    let equations: Vec<(u64, Vec<u64>)> = input
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.trim().split(": ");
            let value = parts.next().unwrap().parse().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (value, numbers)
        })
        .collect();

    // Sum all values that are possible
    equations
        .iter()
        .filter(|(value, numbers)| is_possible(*value, numbers))
        .map(|(value, _)| value)
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day07.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20";
        assert_eq!(solve(input), 11387);
    }
}
