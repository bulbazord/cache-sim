// put actual stuff here

pub use self::cachedefs::{AccessType, CacheStats, CacheBlock, Cache};

pub mod cachedefs;

#[derive(Debug)]
pub struct CacheSystem {
    pub stats: CacheStats,
    pub l1: Cache,
}

impl CacheSystem {
    pub fn new(c1: u64, b1: u64, s1: u64, v: u64, c2: u64, b2: u64, s2: u64) -> CacheSystem {
        CacheSystem {
            stats: CacheStats::new(),
            l1: Cache::new(c1, b1, s1),
        }
    }

    pub fn cache_access(&mut self, mode: AccessType, address: u64) {
        //println!("Trying to {:?} at address {:#X}", mode, address);
        let in_l1 = self.search_l1(mode, address);
        if !in_l1 {
            self.move_to_l1(mode, address);
        }
    }

    pub fn complete_cache(&mut self) {
        println!("Completing the cache!");
    }

    pub fn print_statistics(&self) {
        println!("Cache Statistics");
        println!("Accesses: {}", self.stats.accesses);
        println!("Accesses to L2: {}", self.stats.accesses_l2);
        println!("Accesses to VC: {}", self.stats.accesses_vc);
        println!("Reads: {}", self.stats.reads);
        println!("Read misses to L1: {}", self.stats.read_misses_l1);
        println!("Read misses to L2: {}", self.stats.read_misses_l2);
        println!("Writes: {}", self.stats.writes);
        println!("Write misses to L1: {}", self.stats.write_misses_l1);
        println!("Write misses to L2: {}", self.stats.write_misses_l2);
        println!("Write backs from L1: {}", self.stats.write_back_l1);
        println!("Write backs from L2: {}", self.stats.write_back_l2);
        println!("L1 victims hit in victim hits: {}", self.stats.victim_hits);
        println!("Average access time: {}", self.stats.avg_access_time_l1);
    }

    fn search_l1(&mut self, mode: AccessType, address: u64) -> bool {
        let ref mut stats = self.stats;
        let ref mut cache = self.l1;
        stats.accesses += 1;

        let mut found = false;
        let mut hot_block_index = -1i64;
        let index = (address >> cache.b) & ((1u64 << cache.indexbits) - 1);
        let tag = address >> (cache.b + cache.indexbits);

        let selected_set = cache.sets.get_mut(index as usize);
        if let Some(set) = selected_set {
            for (count, block) in set.into_iter().enumerate() {
                if tag == block.tag {
                    hot_block_index = count as i64;
                    found = true;
                    break;
                }
            }
            if found {
                let mut hot_block = set.remove(hot_block_index as usize).unwrap();
                if let AccessType::Write = mode {
                    hot_block.dirty = true;
                }
                set.push_back(hot_block);
            }
        } else {
            panic!("Address index out of bounds in L1! Panic!")
        }


        if let AccessType::Read = mode {
            stats.reads += 1;
            if !found {
                stats.read_misses_l1 += 1;
            }
        } else {
            stats.writes += 1;
            if !found {
                stats.write_misses_l1 += 1;
            }
        }

        found
    }

    fn move_to_l1(&mut self, mode: AccessType, address: u64) {
        let ref mut stats = self.stats;
        let ref mut cache = self.l1;

        let index = (address >> cache.b) & ((1u64 << cache.indexbits) - 1);
        let tag = address >> (cache.b + cache.indexbits);
        let block_in = CacheBlock {
            address: address,
            tag: tag,
            dirty: match mode {
                AccessType::Read => { false },
                AccessType::Write => { true },
            },
        };

        let selected_set = cache.sets.get_mut(index as usize);
        if let Some(set) = selected_set {
            if set.len() >= cache.max_blocks_per_set as usize {
                let evicted_block = set.pop_front().unwrap();
                if evicted_block.dirty {
                    stats.write_back_l1 += 1;
                }
            }
            set.push_back(block_in);
        } else {
            panic!("Address index out of bounds in L1! Panic!")
        }
    }
}
