#[derive(Debug, Clone, Copy)]
pub enum DiskEntry {
    FreeSpace { size: usize },
    File { id: usize, size: usize },
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<DiskEntry> {
    input
        .char_indices()
        .fold(Vec::new(), |mut memory, (id, size)| {
            let size = size.to_digit(10).unwrap() as usize;
            if id % 2 == 0 {
                let id = id / 2;
                memory.push(DiskEntry::File { id, size });
            } else {
                memory.push(DiskEntry::FreeSpace { size });
            }
            memory
        })
}

#[aoc(day9, part1)]
pub fn part1(input: &[DiskEntry]) -> usize {
    let mut memory = input.to_vec();
    let mut clean_memory = Vec::new();

    let mut write_idx = 0;
    while write_idx < memory.len() {
        let block = &memory[write_idx];
        match *block {
            DiskEntry::File { id, size } => clean_memory.push(DiskEntry::File { id, size }),
            DiskEntry::FreeSpace { size: free_size } => {
                fill_freespace(&mut memory, free_size, write_idx, &mut clean_memory);
            }
        }
        write_idx += 1;
    }
    check_sum(&clean_memory)
}

fn fill_freespace(
    memory: &mut Vec<DiskEntry>,
    mut free_size: usize,
    write_idx: usize,
    clean_memory: &mut Vec<DiskEntry>,
) {
    let mut read_idx = memory.len() - 1;

    while free_size > 0 && read_idx > write_idx {
        if let DiskEntry::File {
            id,
            size: file_size,
        } = memory[read_idx]
        {
            if file_size <= free_size {
                clean_memory.push(DiskEntry::File {
                    id,
                    size: file_size,
                });
                free_size -= file_size;
                memory.remove(read_idx);
                read_idx -= 1;
            } else {
                clean_memory.push(DiskEntry::File {
                    id,
                    size: free_size,
                });
                memory[read_idx] = DiskEntry::File {
                    id,
                    size: file_size - free_size,
                };
                free_size = 0;
            }
        } else {
            read_idx -= 1;
        }
    }
}

fn check_sum(memory: &[DiskEntry]) -> usize {
    memory
        .iter()
        .fold((0, 0), |(mut checksum, mut position), de| {
            if let DiskEntry::File { id, size } = de {
                for _ in 0..*size {
                    checksum += id * position;
                    position += 1;
                }
            } else if let DiskEntry::FreeSpace { size } = de {
                position += size;
            }
            (checksum, position)
        })
        .0
}

#[aoc(day9, part2)]
pub fn part2(input: &[DiskEntry]) -> usize {
    let mut memory = input.to_vec();
    let mut i = memory.len() - 1;
    while i > 0 {
        if let DiskEntry::File {
            id,
            size: file_size,
        } = memory[i]
        {
            let mut insertion_idx = 0;

            loop {
                if let DiskEntry::FreeSpace { size: free_size } = memory[insertion_idx] {
                    if free_size > file_size {
                        memory[i] = DiskEntry::FreeSpace { size: file_size };
                        memory[insertion_idx] = DiskEntry::File {
                            id,
                            size: file_size,
                        };
                        memory.insert(
                            insertion_idx + 1,
                            DiskEntry::FreeSpace {
                                size: free_size - file_size,
                            },
                        );
                        break;
                    }

                    if free_size == file_size {
                        memory[i] = DiskEntry::FreeSpace { size: file_size };
                        memory[insertion_idx] = DiskEntry::File {
                            id,
                            size: file_size,
                        };
                        break;
                    }
                }

                if insertion_idx == i {
                    break;
                }
                insertion_idx += 1;
            }
        }
        i -= 1;
    }
    check_sum(&memory)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "2333133121414131402";
        assert_eq!(1928, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "2333133121414131402";
        assert_eq!(2858, part2(&input_generator(input)));
    }
}
