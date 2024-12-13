use std::{collections::HashMap, vec};

#[derive(Debug, Clone, Copy)]
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
        for i in 0..antennas.len() {
            for j in 0..antennas.len() {
                if i != j {
                    let a = &antennas[i];
                    let b = &antennas[j];

                    let mut node = *b;
                    while node.is_positive() {
                        if let Some(row) = antinodes.get_mut(node.y as usize) {
                            if let Some(b) = row.get_mut(node.x as usize) {
                                *b = true;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }

                        node = Point {
                            x: node.x + (b.x - a.x),
                            y: node.y + (b.y - a.y),
                        };
                    }
                }
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
            T.........
            ...T......
            .T........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........";
        assert_eq!(solve(input), 9);
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
        assert_eq!(solve(input), 34);
    }
}
