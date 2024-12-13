#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    #[allow(dead_code)]
    fn as_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Object => '#',
            Self::Guard(Direction::Up) => '^',
            Self::Guard(Direction::Down) => 'v',
            Self::Guard(Direction::Left) => '<',
            Self::Guard(Direction::Right) => '>',
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

fn count_visited(visited: &[Vec<bool>]) -> u32 {
    visited.iter().flatten().filter(|&&v| v).count() as u32
}

#[allow(dead_code)]
fn draw_map(map: &Map, visited: &[Vec<bool>]) {
    println!("\nVisited: {}", count_visited(visited));
    for (y, row) in map.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            print!(
                "{}",
                if !visited[y][x] || matches!(point, Point::Guard(_)) {
                    point.as_char()
                } else {
                    'X'
                }
            );
        }
        println!();
    }
}

fn solve(input: &str) -> u32 {
    let mut map: Map = input
        .trim()
        .split("\n")
        .map(|line| line.trim().chars().map(Point::from_char).collect())
        .collect();

    let (mut guard_dir, mut guard_pos) = find_guard(&map);

    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    visited[guard_pos.1][guard_pos.0] = true;

    while let Some((point, (x, y))) = get_point_in_direction(&map, guard_pos, guard_dir) {
        // draw_map(&map, &visited);
        map[guard_pos.1][guard_pos.0] = Point::Empty;

        match point {
            Point::Empty => {
                visited[y][x] = true;
                guard_pos = (x, y);
                map[y][x] = Point::Guard(guard_dir);
            }
            Point::Object => {
                guard_dir = guard_dir.rotate90();
            }
            Point::Guard(_) => unreachable!(),
        }
    }
    // draw_map(&map, &visited);

    count_visited(&visited)
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
        assert_eq!(solve(input), 41);
    }
}
