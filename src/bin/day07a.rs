fn is_possible(value: u64, numbers: &[u64]) -> bool {
    /* Split the numbers such that the first number is the initial value
     * and the rest of the numbers have a corresponding operation which is
     * used to combine them with the current value.
     */
    let init_val = numbers[0];
    let numbers = &numbers[1..];

    // Try all possible combinations of + and * operations
    let combinations = 2usize.pow(numbers.len() as u32);
    for combination in 0..combinations {
        let mut cur_val = init_val;
        for (i, number) in numbers.iter().enumerate() {
            // If bit i is 0, add the number, otherwise multiply
            if combination & (1 << i) == 0 {
                cur_val += number;
            } else {
                cur_val *= number;
            }

            /* Break if the current value is already larger than the
             * target value because it will only get larger. This also
             * avoids overflow errors.
             */
            if cur_val > value {
                break;
            }
        }
        if cur_val == value {
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
        assert_eq!(solve(input), 3749);
    }
}
