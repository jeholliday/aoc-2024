type Map = Vec<Vec<u8>>;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

fn adjacent(map: &Map, point: &Point) -> Vec<Point> {
    let mut adj = Vec::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (i, j) in directions.iter() {
        let x = point.x + i;
        let y = point.y + j;
        if x >= 0 && x < map[0].len() as isize && y >= 0 && y < map.len() as isize {
            adj.push(Point { x, y });
        }
    }

    adj
}

fn next(map: &Map, point: &Point) -> Vec<Point> {
    let cur = map[point.y as usize][point.x as usize];
    adjacent(map, point)
        .into_iter()
        .filter(|p| map[p.y as usize][p.x as usize] == cur + 1)
        .collect()
}

fn score(map: &Map, trailhead: &Point) -> u32 {
    assert!(map[trailhead.y as usize][trailhead.x as usize] == 0);
    let mut score = 0;
    let mut to_visit = vec![trailhead.clone()];
    while let Some(cur) = to_visit.pop() {
        if map[cur.y as usize][cur.x as usize] == 9 {
            score += 1;
            continue;
        }
        to_visit.extend(next(map, &cur));
    }
    score
}

fn solve(input: &str) -> u32 {
    let map: Vec<Vec<u8>> = input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap_or(u8::MAX as u32) as u8)
                .collect()
        })
        .collect();

    let mut trailheads = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                trailheads.push(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }

    trailheads.iter().map(|point| score(&map, point)).sum()
}

fn main() {
    let input = include_str!("../../inputs/day10.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9";
        assert_eq!(solve(input), 2);

        let input = "
            ..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....";
        assert_eq!(solve(input), 13);

        let input = "
            012345
            123456
            234567
            345678
            4.6789
            56789.";
        assert_eq!(solve(input), 227);

        let input = "
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732";
        assert_eq!(solve(input), 81);
    }
}
