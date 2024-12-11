use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate90(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Point {
    Empty,
    Object,
    Guard(Direction),
}

impl Point {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Object,
            '^' => Self::Guard(Direction::Up),
            'v' => Self::Guard(Direction::Down),
            '<' => Self::Guard(Direction::Left),
            '>' => Self::Guard(Direction::Right),
            _ => panic!("Invalid character"),
        }
    }
}

type Map = Vec<Vec<Point>>;

fn find_guard(map: &Map) -> (Direction, (usize, usize)) {
    map.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, point)| {
                if let Point::Guard(dir) = point {
                    Some((*dir, (x, y)))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn get_point_in_direction(
    map: &Map,
    (x, y): (usize, usize),
    dir: Direction,
) -> Option<(Point, (usize, usize))> {
    match dir {
        Direction::Up if y > 0 => Some((map[y - 1][x], (x, y - 1))),
        Direction::Down if y < map.len() - 1 => Some((map[y + 1][x], (x, y + 1))),
        Direction::Left if x > 0 => Some((map[y][x - 1], (x - 1, y))),
        Direction::Right if x < map[y].len() - 1 => Some((map[y][x + 1], (x + 1, y))),
        _ => None,
    }
}

fn causes_loop(
    map: &Map,
    mut guard_dir: Direction,
    mut guard_pos: (usize, usize),
    obstacle: (usize, usize),
) -> bool {
    let mut visited: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    visited.insert(guard_pos, vec![guard_dir]);

    while let Some((mut point, (x, y))) = get_point_in_direction(&map, guard_pos, guard_dir) {
        if (x, y) == obstacle {
            point = Point::Object;
        }
        match point {
            Point::Empty => {
                if let Some(dirs) = visited.get_mut(&(x, y)) {
                    if dirs.contains(&guard_dir) {
                        return true;
                    }
                    dirs.push(guard_dir);
                } else {
                    visited.insert((x, y), vec![guard_dir]);
                }
                guard_pos = (x, y);
            }
            Point::Object => {
                guard_dir = guard_dir.rotate90();
            }
            Point::Guard(_) => unreachable!(),
        }
    }

    false
}

fn solve(input: &str) -> u32 {
    let mut map: Map = input
        .trim()
        .split("\n")
        .map(|line| line.trim().chars().map(Point::from_char).collect())
        .collect();
    let (guard_dir, guard_pos) = find_guard(&map);
    map[guard_pos.1][guard_pos.0] = Point::Empty;

    let mut count = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if (x, y) == guard_pos {
                continue;
            }
            if let Point::Empty = point {
                if causes_loop(&map, guard_dir, guard_pos, (x, y)) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("../../inputs/day06.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...";
        assert_eq!(solve(input), 6);
    }
}
