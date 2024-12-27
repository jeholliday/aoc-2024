use std::collections::HashSet;

fn solve(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    /* For each location in grid, check if it has been visited. If it has not,
     * assume that it is the first location in a new region. Grow that region
     * by checking all adjacent locations for a matching plant type.
     */
    let mut ans = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if visited[y][x] {
                continue;
            }
            let mut area = 0;
            let mut perimeter = 0;

            /* Iterate over adjacent locations with same plant type until there
             * are no more.
             */
            let mut to_visit: HashSet<(usize, usize)> = HashSet::new();
            to_visit.insert((x, y));
            while let Some((x, y)) = to_visit.iter().next().cloned() {
                to_visit.remove(&(x, y));
                visited[y][x] = true;
                area += 1;
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let a = x as isize + dx;
                    let b = y as isize + dy;
                    if a < 0 || b < 0 {
                        perimeter += 1;
                        continue;
                    }
                    let (a, b) = (a as usize, b as usize);
                    if let Some(cc) = map.get(b).and_then(|l| l.get(a)) {
                        if *c == *cc {
                            if !visited[b][a] {
                                to_visit.insert((a, b));
                            }
                        } else {
                            perimeter += 1;
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }

            ans += area * perimeter;
        }
    }

    ans
}

fn main() {
    let input = include_str!("../../inputs/day12.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            AAAA
            BBCD
            BBCC
            EEEC";
        assert_eq!(solve(input), 140);

        let input = "
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO";
        assert_eq!(solve(input), 772);

        let input = "
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE";
        assert_eq!(solve(input), 1930);
    }
}
