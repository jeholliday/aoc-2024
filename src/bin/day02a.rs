/// Trend up or down with previous value
enum Trend {
    Increasing(u32),
    Decreasing(u32),
    Any(u32),
    Unknown,
}

const MIN_DIFF: u32 = 1;
const MAX_DIFF: u32 = 3;

fn is_safe_with_trend(report: &[u32], trend: Trend) -> bool {
    if report.is_empty() {
        return true;
    }

    if let Trend::Unknown = trend {
        return is_safe_with_trend(&report[1..], Trend::Any(report[0]));
    }

    let cur = report[0];
    let prev = match trend {
        Trend::Increasing(prev) => prev,
        Trend::Decreasing(prev) => prev,
        Trend::Any(prev) => prev,
        Trend::Unknown => unreachable!(),
    };

    // Check if the current value is consistent with the trend
    if let Trend::Increasing(_) = trend {
        if cur < prev {
            return false;
        }
    }
    if let Trend::Decreasing(_) = trend {
        if cur > prev {
            return false;
        }
    }

    // Check if the difference between the current and previous values is within the allowed range
    let diff = num::abs(cur as isize - prev as isize) as u32;
    if !(MIN_DIFF..=MAX_DIFF).contains(&diff) {
        return false;
    }

    // Check if the remaining values safely follow the trend
    match trend {
        Trend::Increasing(_) => is_safe_with_trend(&report[1..], Trend::Increasing(cur)),
        Trend::Decreasing(_) => is_safe_with_trend(&report[1..], Trend::Decreasing(cur)),
        Trend::Any(_) => {
            if cur > prev {
                is_safe_with_trend(&report[1..], Trend::Increasing(cur))
            } else {
                is_safe_with_trend(&report[1..], Trend::Decreasing(cur))
            }
        }
        Trend::Unknown => unreachable!(),
    }
}

fn is_safe(report: &[u32]) -> bool {
    is_safe_with_trend(report, Trend::Unknown)
}

fn solve(input: &str) -> u32 {
    let input: Vec<&str> = input.trim().split("\n").map(str::trim).collect();
    let reports: Vec<Vec<u32>> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect();

    // Get number of safe reports
    reports.iter().filter(|r| is_safe(r)).count() as u32
}

fn main() {
    let input = include_str!("../../inputs/day02.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9";
        assert_eq!(solve(input), 2);
    }
}
