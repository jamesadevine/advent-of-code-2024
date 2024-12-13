use anyhow::Result;
use std::{fmt::Display, fs::File, io::Read};

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day9\\day9.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Debug, Clone)]
struct Block {
    id: i32,
}

impl Block {
    fn from_id(id: i32, size: usize) -> Vec<Self> {
        (0..size).map(|_| Block { id }).collect()
    }

    fn is_free(&self) -> bool {
        self.id == -1
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_free() {
            write!(f, ".")
        } else {
            write!(f, "{}", self.id)
        }
    }
}

#[derive(Debug)]
struct Partition {
    blocks: Vec<Block>,
}

impl Partition {
    fn new(id: Option<usize>, size: usize) -> Self {
        match id {
            Some(id) => Partition {
                blocks: Block::from_id(id as i32, size),
            },
            None => Partition {
                blocks: Block::from_id(-1, size),
            },
        }
    }
    fn size(&self) -> usize {
        self.blocks.len()
    }

    fn used(&self) -> usize {
        self.blocks.iter().filter(|b| b.id != -1).count()
    }

    fn free(&self) -> usize {
        self.size() - self.used()
    }

    fn contiguous_space(&self, start_idx: usize) -> usize {
        let mut idx = start_idx;
        let mut free_space = 0;

        while idx < self.blocks.len() && self.blocks[idx].is_free() {
            idx += 1;
            free_space += 1;
        }
        free_space
    }

    fn contiguous_free_space_for_size(&self, size: usize) -> Option<usize> {
        for i in 0..self.blocks.len() {
            if self.blocks[i].is_free() {
                if self.contiguous_space(i) >= size {
                    return Some(i);
                }
            }
        }
        None
    }

    fn take_block(&mut self) -> Option<Block> {
        for i in 0..self.blocks.len() {
            if !self.blocks[i].is_free() {
                let ret = self.blocks[i].clone();
                self.blocks[i].id = -1;
                return Some(ret);
            }
        }
        None
    }

    fn add_block(&mut self, block: Block) {
        for i in 0..self.blocks.len() {
            if self.blocks[i].id == -1 {
                self.blocks[i] = block;
                return;
            }
        }
    }
}

impl Display for Partition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}

struct Disk {
    partitions: Vec<Partition>,
}

impl Disk {
    fn from_string(representation: String) -> Self {
        let mut partitions = Vec::new();

        let mut is_file = true;
        let mut idx = 0;

        for c in representation.bytes() {
            let file_idx = if is_file { Some(idx) } else { None };

            partitions.push(Partition::new(
                file_idx,
                (c as char).to_string().parse::<usize>().unwrap(),
            ));

            if is_file {
                idx += 1;
            }

            is_file = !is_file;
        }

        Disk { partitions }
    }

    fn find_free_block(&self) -> Option<usize> {
        for (idx, partition) in self.partitions.iter().enumerate() {
            if partition.used() < partition.size() {
                return Some(idx);
            }
        }

        None
    }

    fn last_used_partition(&self) -> Option<usize> {
        for (idx, partition) in self.partitions.iter().enumerate().rev() {
            if partition.used() > 0 {
                return Some(idx);
            }
        }
        None
    }

    fn realloc_block(&mut self, from_partition_idx: usize, to_partition_idx: usize) {
        let block = self.partitions[from_partition_idx].take_block().unwrap();
        self.partitions[to_partition_idx].add_block(block);
    }

    fn realloc_partition(&mut self, from_partition_idx: usize, to_partition_idx: usize) {
        let block_idx = self.partitions[to_partition_idx]
            .contiguous_free_space_for_size(self.partitions[from_partition_idx].size())
            .unwrap();
        for i in 0..self.partitions[from_partition_idx].size() {
            self.partitions[to_partition_idx].blocks[block_idx + i] =
                self.partitions[from_partition_idx].blocks[i].clone();
            self.partitions[from_partition_idx].blocks[i].id = -1;
        }
    }

    fn compress_as_blocks(&mut self) {
        loop {
            let last_used_partition_idx = match self.last_used_partition() {
                Some(idx) => idx,
                None => break,
            };

            let free_block_idx = match self.find_free_block() {
                Some(idx) => idx,
                None => break,
            };

            if last_used_partition_idx <= free_block_idx {
                break;
            }

            self.realloc_block(last_used_partition_idx, free_block_idx);
        }
    }

    fn last_used_file(&self) -> Option<usize> {
        for (idx, partition) in self.partitions.iter().enumerate().rev() {
            if partition.used() == partition.size() {
                return Some(idx);
            }
        }
        None
    }

    fn file_by_id(&self, id: i32) -> Option<usize> {
        for (idx, partition) in self.partitions.iter().enumerate() {
            if partition.blocks.len() == 0 {
                continue;
            }
            if partition.blocks.iter().all(|b| b.id == id) {
                return Some(idx);
            }
        }
        None
    }

    fn find_free_partition_for_size(&self, partitions: &[Partition], size: usize) -> Option<usize> {
        for (idx, partition) in partitions.iter().enumerate() {
            if partition.free() > 0 && partition.contiguous_free_space_for_size(size).is_some() {
                return Some(idx);
            }
        }
        None
    }

    fn compress_as_partitions(&mut self) {
        let initial_partition = self.last_used_file().unwrap();
        let last_file_partition = &self.partitions[initial_partition];
        let mut last_block_id = last_file_partition.blocks[0].id;

        while last_block_id > -1 {
            let last_used_partition_idx = match self.file_by_id(last_block_id) {
                Some(idx) => idx,
                None => break,
            };

            let free_block_idx = match self.find_free_partition_for_size(
                &self.partitions[0..last_used_partition_idx],
                self.partitions[last_used_partition_idx].used(),
            ) {
                Some(idx) => idx,
                None => {
                    last_block_id -= 1;
                    continue;
                }
            };
            self.realloc_partition(last_used_partition_idx, free_block_idx);
            last_block_id -= 1;
        }
    }

    fn checksum_blocks(&self) -> usize {
        let mut checksum = 0;
        let mut idx = 0;
        for partition in &self.partitions {
            for block in &partition.blocks {
                if !block.is_free() {
                    checksum += idx * (block.id as usize);
                }
                idx += 1;
            }
        }
        checksum
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for partition in &self.partitions {
            write!(f, "{}", partition)?;
        }
        Ok(())
    }
}

fn main() {
    let data = retrieve_data().unwrap();

    let mut disk = Disk::from_string(data);
    println!("{}", disk);
    disk.compress_as_partitions();
    println!("{}", disk);
    println!("{}", disk.checksum_blocks());
}
