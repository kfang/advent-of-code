use std::fs;

enum Block {
    File(u64),
    FreeSpace,
}

#[derive(Debug)]
struct FileInfo {
    start_idx: usize,
    block_count: i32,
}

#[derive(Debug)]
struct Sector {
    start_idx: usize,
    block_count: i32
}

struct Disk {
    blocks: Vec<Block>,
    files: Vec<FileInfo>,
    free_spaces: Vec<Sector>,
    last_file_idx: usize,
}

impl Disk {
    fn swap(&mut self, idx0: usize, idx1: usize) {
        self.blocks.swap(idx0, idx1);

        while self.last_file_idx > 0 {
            let block = &self.blocks[self.last_file_idx];
            match block {
                Block::File(_file_id) => { break; },
                _ => { 
                    self.last_file_idx -= 1;
                },
            };
        }
    }

    fn checksum(&self) -> u64 {
        let mut total: u64 = 0;

        self.blocks.iter().enumerate().for_each(|(idx, block)| {
            match block {
                Block::File(file_id) => { total += idx as u64 * file_id},
                _ => (),
            };
        });

        return total;
    }
}

fn read_disk(file_path: &str) -> Disk {
    let mut blocks: Vec<Block> = Vec::new();
    let mut files: Vec<FileInfo> = Vec::new();
    let mut free_spaces: Vec<Sector> = Vec::new();
    let mut last_file_idx: usize = 0;

    fs::read_to_string(file_path).expect("unable to read file")
        .chars()
        .filter_map(|c| c.to_string().parse::<i32>().map(|i| Some(i)).unwrap_or(None))
        .enumerate()
        .for_each(|(idx, block_count)| {
            let start_idx = blocks.len();
            let sector = Sector { start_idx, block_count };
            
            if idx % 2 == 0 {
                let file_id = idx as u64 / 2;
                let file_info = FileInfo { start_idx, block_count };
                files.push(file_info);
                
                for _c in 0..block_count {
                    blocks.push(Block::File(file_id));
                }
                last_file_idx = blocks.len() - 1;
            } else {
                free_spaces.push(sector);
                for _c in 0..block_count {
                    blocks.push(Block::FreeSpace);
                }
            }
        });

    files.reverse();

    return Disk { blocks, files, free_spaces, last_file_idx };
}

pub fn day_9_1(file_path: &str) -> u64 {
    let mut disk = read_disk(file_path);

    for idx in 0..disk.blocks.len() {
        // everything after is free space, no need to keep checking
        if idx >= disk.last_file_idx {
            break;
        }

        let block = &disk.blocks[idx];
        match block {
            Block::FreeSpace => {
                disk.swap(disk.last_file_idx, idx);
            },
            _ => (),
        }
    }

    return disk.checksum();
}

pub fn day_9_2(file_path: &str) -> u64 {
    let mut disk = read_disk(file_path);

    for file_idx in 0..disk.files.len() {
        let file = &mut disk.files[file_idx];

        let free_idx = disk.free_spaces.iter().position(|sec| {
            return sec.block_count >= file.block_count
                && sec.start_idx < file.start_idx
        });

        if free_idx.is_some() {
            let free_space_idx = free_idx.unwrap();
            let block_count = file.block_count;
            let start_x = file.start_idx;
            let free = &mut disk.free_spaces[free_space_idx];

            for offset in 0..block_count as usize {
                let idx0 = start_x + offset;
                let idx1 = free.start_idx + offset;
                disk.blocks.swap(idx0, idx1);
            }
    
            file.start_idx = free_space_idx;

            free.block_count -= block_count;
            free.start_idx += block_count as usize;
        }
    }

    return disk.checksum();
}


#[cfg(test)]
mod tests {
    use super::day_9_1;
    use super::day_9_2;

    #[test]
    fn day_9_1_sample() {
        let result = day_9_1("inputs/d9_1_sample.txt");
        assert_eq!(result, 1928);
    }
    
    #[test]
    fn day_9_1_run() {
        let result = day_9_1("inputs/d9_1.txt");
        println!("{result}");
        assert_eq!(result, 6201130364722);
    }
    
    #[test]
    fn day_9_2_sample() {
        let result = day_9_2("inputs/d9_1_sample.txt");
        assert_eq!(result, 2858);
    }
    
    #[test]
    fn day_9_2_run() {
        let result = day_9_2("inputs/d9_1.txt");
        println!("{result}");
        assert_eq!(result, 2858);
    }
}
