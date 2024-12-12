use std::{collections::HashMap, vec};

use itertools::Itertools;

struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    fn is_positive(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }
}

fn solve(input: &str) -> usize {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let input: Vec<&str> = input.trim().split("\n").map(str::trim).collect();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push(Point::new(x, y));
            }
        }
    }

    let mut antinodes: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
    for (_freq, antennas) in antennas {
        for pair in antennas.iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);
            let node = Point {
                x: b.x + (b.x - a.x),
                y: b.y + (b.y - a.y),
            };
            if node.is_positive() {
                antinodes
                    .get_mut(node.y as usize)
                    .and_then(|row| row.get_mut(node.x as usize))
                    .map(|b| *b = true);
            }
            let node = Point {
                x: a.x + (a.x - b.x),
                y: a.y + (a.y - b.y),
            };
            if node.is_positive() {
                antinodes
                    .get_mut(node.y as usize)
                    .and_then(|row| row.get_mut(node.x as usize))
                    .map(|b| *b = true);
            }
        }
    }

    antinodes.iter().flatten().filter(|&&b| b).count()
}

fn main() {
    let input = include_str!("../../inputs/day08.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
            ";
        assert_eq!(solve(input), 14);
    }
}
