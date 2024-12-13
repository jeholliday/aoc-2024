const WORD: &str = "XMAS";
const WORD_LEN: usize = WORD.len();

type Grid = Vec<Vec<(char, bool)>>;

fn is_word_right(grid: &mut Grid, x: usize, y: usize) -> bool {
    if x + WORD_LEN > grid[y].len() {
        return false;
    }
    for (i, c) in WORD.chars().enumerate() {
        if grid[y][x + i].0 != c {
            return false;
        }
    }

    for i in 0..WORD_LEN {
        grid[y][x + i].1 = true;
    }

    true
}

fn is_word_down_right(grid: &mut Grid, x: usize, y: usize) -> bool {
    if x + WORD_LEN > grid[y].len() || y + WORD_LEN > grid.len() {
        return false;
    }
    for (i, c) in WORD.chars().enumerate() {
        if grid[y + i][x + i].0 != c {
            return false;
        }
    }

    for i in 0..WORD_LEN {
        grid[y + i][x + i].1 = true;
    }

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
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if is_word_right(&mut grid, x, y) {
                    count += 1;
                }
                if is_word_down_right(&mut grid, x, y) {
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
        assert_eq!(solve(input), 18);
    }
}
