use std::cmp::Ordering;

use regex::Regex;

struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)"
    )
    .unwrap();
}

fn solve(input: &str, iters: usize, width: usize, height: usize) -> usize {
    let mut robots: Vec<Robot> = REGEX
        .captures_iter(input)
        .map(|c| {
            let (_, [x, y, vx, vy]) = c.extract();
            Robot {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect();

    let width = width as isize;
    let height = height as isize;
    for _ in 0..iters {
        for robot in robots.iter_mut() {
            robot.x += robot.vx;
            if robot.x < 0 {
                robot.x += width;
            } else if robot.x >= width {
                robot.x -= width;
            }

            robot.y += robot.vy;
            if robot.y < 0 {
                robot.y += height;
            } else if robot.y >= height {
                robot.y -= height;
            }
        }
    }

    let mid_x = width / 2;
    let mid_y = height / 2;

    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
    for robot in robots.iter() {
        match (robot.x.cmp(&mid_x), robot.y.cmp(&mid_y)) {
            (Ordering::Greater, Ordering::Greater) => a += 1,
            (Ordering::Greater, Ordering::Less) => b += 1,
            (Ordering::Less, Ordering::Greater) => c += 1,
            (Ordering::Less, Ordering::Less) => d += 1,
            _ => {}
        }
    }
    a * b * c * d
}

fn main() {
    let input = include_str!("../../inputs/day14.txt");
    let ans = solve(input, 100, 101, 103);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3";
        assert_eq!(solve(input, 100, 11, 7), 12);
    }
}
