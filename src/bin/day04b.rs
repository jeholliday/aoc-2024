type Grid = Vec<Vec<(char, bool)>>;

fn is_xmas(grid: &mut Grid, x: usize, y: usize) -> bool {
    if grid[y][x].0 != 'A' {
        return false;
    }
    if grid[y - 1][x - 1].0 != 'M' {
        return false;
    }
    if grid[y - 1][x + 1].0 != 'M' {
        return false;
    }
    if grid[y + 1][x - 1].0 != 'S' {
        return false;
    }
    if grid[y + 1][x + 1].0 != 'S' {
        return false;
    }

    grid[y][x].1 = true;
    grid[y - 1][x - 1].1 = true;
    grid[y - 1][x + 1].1 = true;
    grid[y + 1][x - 1].1 = true;
    grid[y + 1][x + 1].1 = true;

    true
}

fn rotate_grid(grid: &Grid) -> Grid {
    // Rotate grid 90 degrees clockwise
    let mut new_grid = vec![vec![(0 as char, false); grid.len()]; grid[0].len()];
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            new_grid[x][grid.len() - 1 - y] = *value;
        }
    }
    new_grid
}

fn print_grid(grid: &Grid) {
    for row in grid {
        for (c, used) in row {
            if *used {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve(input: &str) -> u32 {
    let mut grid: Grid = input
        .split_whitespace()
        .map(|line| line.chars().map(|c| (c, false)).collect())
        .collect();
    let mut count = 0;
    for _ in 0..4 {
        for y in 1..grid.len() - 1 {
            for x in 1..grid[y].len() - 1 {
                if is_xmas(&mut grid, x, y) {
                    count += 1;
                }
            }
        }
        grid = rotate_grid(&grid);
    }
    print_grid(&grid);
    count
}

fn main() {
    let input = include_str!("../../inputs/day04.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX";
        assert_eq!(solve(input), 9);
    }
}
