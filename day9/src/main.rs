fn main() {
    let input = util::read_input("day9/input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

use std::fmt::Debug;

fn part1(data: &str) -> String {
    let mut disk = Disk::parse(data);

    disk.reallocate();

    format!("{}", disk.checksum())
}


fn part2(data: &str) -> String {
    let mut disk = SizedDisk::parse(data);

    disk.reallocate();

    format!("{}", disk.checksum())
}

#[derive(Debug, Clone)]
enum Block {
    Free,
    File(u32),
}

struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    fn parse(data: &str) -> Disk {
        let mut blocks = Vec::new();
        let mut file = true;
        let mut file_id = 0;

        for c in data.trim().chars() {
            let size = c.to_digit(10).unwrap();
            if file {
                for _ in 0..size {
                    blocks.push(Block::File(file_id));
                }

                file_id += 1;
                file = false;
            } else {
                for _ in 0..size {
                    blocks.push(Block::Free);
                }
                file = true;
            }
        }

        Disk { blocks }
    }

    fn reallocate(&mut self) {
        for i in (0..self.blocks.len()).rev() {
            if let Block::File(id) = self.blocks[i] {
                let next_free = &self.blocks[0..i].iter().position(|b| {
                    if let Block::Free = b {
                        true
                    } else {
                        false
                    }
                });
                if let Some(j) = next_free {
                    self.blocks[*j] = Block::File(id);
                    self.blocks[i] = Block::Free;
                } else {
                    // No more free blocks.
                    return;
                }
            }
        }
    }

    fn checksum(&self) -> u64 {
        let mut result = 0;

        for (i, block) in self.blocks.iter().enumerate() {
            if let Block::File(id) = block {
                result += (i as u64) * (*id as u64);
            }
        }

        result
    }
}

impl Debug for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            match block {
                Block::Free => write!(f, ".")?,
                Block::File(id) => write!(f, "{}", id)?,
            }
        }

        Ok(())
    }
}

struct SizedDisk {
    blocks: Vec<SizedBlock>,
}

impl SizedDisk {
    fn parse(data: &str) -> SizedDisk {
        let mut blocks = Vec::new();
        let mut file = true;
        let mut file_id = 0;

        for c in data.trim().chars() {
            let size = c.to_digit(10).unwrap();
            if file {
                blocks.push(SizedBlock::File(file_id, size as usize));
                file_id += 1;
                file = false;
            } else {
                blocks.push(SizedBlock::Free{size: size as usize, files: vec![]});
                file = true;
            }
        }

        SizedDisk { blocks }
    }

    fn reallocate(&mut self) {
        for i in (0..self.blocks.len()).rev() {
            if let SizedBlock::File(id, size) = self.blocks[i] {
                let next_free = &self.blocks[0..i].iter().position(|b| {
                    if let SizedBlock::Free{size: free_size, files: _} = b {
                        size <= *free_size
                    } else {
                        false
                    }
                });

                if let Some(j) = next_free {
                    if let SizedBlock::Free{size: free_size, files} = &mut self.blocks[*j] {
                        files.push((id, size));
                        *free_size -= size;
                    }
                    self.blocks[i] = SizedBlock::Free{size, files: vec![]};
                }
            }
        }
    }

    fn checksum(&self) -> u64 {
        let mut result = 0;
        let mut pos = 0;

        for block in self.blocks.iter() {
            match block {
                SizedBlock::File(id, size) => {
                    for _ in 0..*size {
                        result += (pos as u64) * (*id as u64);
                        pos += 1;
                    }
                }
                SizedBlock::Free{size, files} => {
                    for (id, size) in files {
                        for _ in 0..*size {
                            result += (pos as u64) * (*id as u64);
                            pos += 1;
                        }
                    }
                    pos += size;
                }
            }
        }

        result
    }
}

impl Debug for SizedDisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            match block {
                SizedBlock::Free{size, files} => {
                    for (id, size) in files {
                        for _ in 0..*size {
                            write!(f, "{}", id)?;
                        }
                    }
                    for _ in 0..*size {
                        write!(f, ".")?;
                    }
                }
                SizedBlock::File(id, size) => {
                    for _ in 0..*size {
                        write!(f, "{}", id)?;
                    }
                },
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
enum SizedBlock {
    Free{
        size: usize,
        files: Vec<(u32, usize)>,
    },
    File(u32, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        2333133121414131402
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "1928");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "2858");
    }
}
