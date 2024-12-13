enum Block {
    File(usize),
    Empty,
}

fn solve(input: &str) -> usize {
    // Parse input
    let mut disk: Vec<Block> = Vec::new();
    let mut file_count = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            // File
            let file_id = file_count;
            file_count += 1;
            disk.extend((0..n).map(|_| Block::File(file_id)));
        } else {
            // Empty
            disk.extend((0..n).map(|_| Block::Empty));
        }
    }

    // Compact disk
    let mut left = 0;
    let mut right = disk.len() - 1;
    while left < right {
        if matches!(disk[right], Block::Empty) {
            right -= 1;
        } else if matches!(disk[left], Block::File(_)) {
            left += 1;
        } else {
            disk.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    disk.iter()
        .enumerate()
        .map(|(i, b)| match b {
            Block::File(file_id) => i * file_id,
            Block::Empty => 0,
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day09.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "2333133121414131402";
        assert_eq!(solve(input), 1928);
    }
}
