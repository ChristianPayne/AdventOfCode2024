#[derive(Debug, Clone, Copy)]
enum Block {
    /// File with an id and length.  
    /// ### id
    /// Refers to the position in the list it was before it was rearranged.  
    /// ### length
    /// Refers to the length of the file.  
    File { id: u64, length: u64 },
    /// A free space block with the length of space
    FreeSpace { length: u64 },
}

impl Block {
    fn get_length(self) -> u64 {
        match self {
            Block::File { id: _, length } => length,
            Block::FreeSpace { length } => length,
        }
    }
}

fn part_1(input: &str) -> i64 {
    // Expand the free space blocks
    // Keep a mut vec that allows us to hold different kinds of blocks.
    let mut file_system: Vec<Block> = vec![];
    for block in parse(input) {
        for _ in 0..block.get_length() {
            // Copying the blocks into the filesystem.
            file_system.push(block);
        }
    }

    // When moving, swap the file blocks into the earliest free space block.
    // We are done when the index of the next free space block is greater than the block we are moving.
    'forward_loop: for forward_index in 0..file_system.len() {
        if let Block::FreeSpace { .. } = &file_system[forward_index] {
            'reverse_loop: for reverse_index in (0..file_system.len()).rev() {
                if let Block::File { .. } = &file_system[reverse_index] {
                    // println!(
                    //     "Swapping file {} ({:?}) with freespace {} ({:?})",
                    //     reverse_index,
                    //     &file_system[reverse_index],
                    //     forward_index,
                    //     &file_system[forward_index]
                    // );
                    file_system.swap(reverse_index, forward_index);
                    break 'reverse_loop;
                }

                if reverse_index <= forward_index {
                    break 'forward_loop;
                }
            }
        }
    }

    // dbg!(&file_system);

    // Run the calculation after the blocks are in the correct order.
    file_system
        .iter()
        .enumerate()
        .fold(0, |acc, (index, block)| {
            if let Block::File { id, length: _ } = block {
                return acc + (index as u64 * id) as i64;
            }

            acc
        })
}
fn parse(input: &str) -> Vec<Block> {
    // Keep an incrementing id.
    // Parse the input and generate a list of blocks.
    let block_numbers = input
        .chars()
        .map(|c| c.to_digit(10).expect("Failed to convert input to u64.") as u64)
        .collect::<Vec<u64>>();

    // Keep state of the file ids. As we find more files, increment by 1;
    let mut file_id: u64 = 0;
    let mut disk_map: Vec<Block> = vec![];

    for (i, num) in block_numbers.into_iter().enumerate() {
        if i % 2 == 0 {
            disk_map.push(Block::File {
                id: file_id,
                length: num,
            });
            file_id += 1;
        } else {
            disk_map.push(Block::FreeSpace { length: num });
        }
    }

    disk_map
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let result = part_1("2333133121414131402");
        assert_eq!(result, 1928);
    }
}
