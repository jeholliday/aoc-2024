#[derive(Clone)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Clone)]
enum Block {
    File(File),
    Empty(usize),
}

fn solve(input: &str) -> usize {
    // Parse input
    let mut disk: Vec<Block> = Vec::new();
    let mut file_count = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            let file_id = file_count;
            file_count += 1;
            disk.push(Block::File(File {
                id: file_id,
                size: n as usize,
            }));
        } else {
            disk.push(Block::Empty(n as usize));
        }
    }

    // Compact disk
    for file_id in (0..file_count).rev() {
        // Find the file on the disk
        let (file_index, block) = disk
            .iter()
            .enumerate()
            .find(|(_, block)| {
                if let Block::File(file) = block {
                    if file.id == file_id {
                        return true;
                    }
                }
                false
            })
            .unwrap();
        let file_size = match block {
            Block::File(file) => file.size,
            _ => unreachable!(),
        };

        // Find first free space large enough to fit file
        for i in 0..file_index {
            if let Block::Empty(empty_size) = disk[i] {
                if empty_size >= file_size {
                    // Shrink the empty space
                    disk[i] = Block::Empty(empty_size - file_size);
                    // Replace file with empty space
                    let file = disk[file_index].clone();
                    disk[file_index] = Block::Empty(file_size);
                    // Insert file at new place
                    disk.insert(i, file);
                    break;
                }
            }
        }
    }

    let mut checksum = 0;
    let mut index = 0;
    for block in disk {
        match block {
            Block::File(file) => {
                for i in index..(index + file.size) {
                    checksum += i * file.id
                }
                index += file.size;
            }
            Block::Empty(size) => index += size,
        }
    }

    checksum
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
        assert_eq!(solve(input), 2858);
    }
}
