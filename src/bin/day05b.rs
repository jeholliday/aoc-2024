use std::vec;

fn solve(input: &str) -> u32 {
    let input: Vec<&str> = input.trim().split("\n\n").collect();
    assert!(input.len() == 2);

    let rules: Vec<(u8, u8)> = input[0]
        .lines()
        .map(|line| {
            let mut parts = line.trim().split("|");
            assert!(parts.clone().count() == 2);
            let a = parts.next().unwrap().parse::<u8>().unwrap();
            let b = parts.next().unwrap().parse::<u8>().unwrap();
            println!("{}|{}", a, b);
            (a, b)
        })
        .collect();

    let mut after: [Vec<u8>; 100] = std::array::from_fn(|_| Vec::new());
    for rule in rules {
        after[rule.1 as usize].push(rule.0);
    }
    for (a, b) in after.iter().enumerate() {
        if b.len() > 0 {
            println!("{}: {:?}", a, b);
        }
    }

    let updates: Vec<Vec<u8>> = input[1]
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|n| n.parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let mut invalid_updates: Vec<Vec<u8>> = vec![];
    for update in updates {
        let mut valid = true;
        for i in 0..update.len() {
            let n = update[i];
            for j in i + 1..update.len() {
                let m = update[j];
                if after[n as usize].contains(&m) {
                    valid = false;
                    println!("{:?} not valid because {} must be after {}", update, n, m);
                    invalid_updates.push(update.clone());
                    break;
                }
            }
            if !valid {
                break;
            }
        }
    }

    let mut ans = 0;
    for mut update in invalid_updates {
        let mut correct = vec![];
        while !update.is_empty() {
            // Find page in update which has no pages which it must be after
            let mut found = false;
            for i in 0..update.len() {
                let n = update[i];
                // Check if n must be after any remaining pages in update
                if !after[n as usize].iter().any(|m| update.contains(m)) {
                    correct.push(n);
                    update.remove(i);
                    found = true;
                    break;
                }
            }
            if !found {
                panic!("No valid page found");
            }
        }
        let mid = correct[correct.len() / 2];
        println!("{}: {:?}", mid, correct);
        ans += mid as u32;
    }
    ans
}

fn main() {
    let input = include_str!("../../inputs/day05.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47";
        assert_eq!(solve(input), 123);
    }
}
