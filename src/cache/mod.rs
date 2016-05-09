// put actual stuff here

pub use self::cachedefs::{AccessType, CacheStats, CacheBlock, Cache, VictimCache};

pub mod cachedefs;

#[derive(Debug)]
pub struct CacheSystem {
    pub stats: CacheStats,
    pub l1: Cache,
    pub vc: VictimCache,
    pub l2: Cache,
}

impl CacheSystem {
    pub fn new(c1: u64, b1: u64, s1: u64, v: u64, c2: u64, b2: u64, s2: u64) -> Self {
        CacheSystem {
            stats: CacheStats::new(),
            l1: Cache::new(c1, b1, s1),
            vc: VictimCache::new(b1, v),
            l2: Cache::new(c2, b2, s2),
        }
    }

    pub fn cache_access(&mut self, mode: AccessType, address: u64) {
        //println!("Trying to {:?} at address {:#X}", mode, address);
        let in_l1 = self.search_l1(mode, address);
        if !in_l1 {
            print!("M1");
            let in_vc = self.search_and_modify_vc(address);
            if !in_vc {
                if self.vc.v > 0 {
                    print!("MV");
                }
                let in_l2 = self.search_l2(mode, address);
            } else {
                println!("HV**");
            }
            self.move_to_l1(mode, address);
        } else {
            if self.vc.v > 0 {
                println!("H1****");
            } else {
                println!("H1**");
            }
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
        self.stats.accesses += 1;

        let mut found = false;
        let mut hot_block_index = -1i64;
        let index = (address >> self.l1.b) & ((1u64 << self.l1.indexbits) - 1);
        let tag = address >> (self.l1.b + self.l1.indexbits);

        let selected_set = self.l1.sets.get_mut(index as usize);
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
            self.stats.reads += 1;
            if !found {
                self.stats.read_misses_l1 += 1;
            }
        } else {
            self.stats.writes += 1;
            if !found {
                self.stats.write_misses_l1 += 1;
            }
        }

        found
    }

    fn move_to_l1(&mut self, mode: AccessType, address: u64) {
        let index = (address >> self.l1.b) & ((1u64 << self.l1.indexbits) - 1);
        let tag = address >> (self.l1.b + self.l1.indexbits);
        let block_in = CacheBlock {
            address: address,
            tag: tag,
            dirty: match mode {
                AccessType::Read => { false },
                AccessType::Write => { true },
            },
        };

        let selected_set = self.l1.sets.get_mut(index as usize);
        if let Some(set) = selected_set {
            if set.len() >= self.l1.max_blocks_per_set as usize {
                let mut evicted_block = set.pop_front().unwrap();
                if evicted_block.dirty {
                    self.stats.write_back_l1 += 1;
                    /*let in_l2 = self.search_l2(AccessType::Write, evicted_block.address);
                    if !in_l2 {
                        self.move_to_l2(AccessType::Write, evicted_block.address);
                    }*/
                }
                evicted_block.dirty = false;

                if self.vc.v > 0 {
                    if self.vc.set.len() == self.vc.v as usize {
                        self.vc.set.pop_front();
                    }
                    evicted_block.tag = evicted_block.address >> self.vc.b;
                    self.vc.set.push_back(evicted_block);
                }
            }
            set.push_back(block_in);
        } else {
            panic!("Address index out of bounds in L1! Panic!")
        }
    }

    fn search_and_modify_vc(&mut self, address: u64) -> bool {
        if self.vc.v == 0 {
            return false;
        }

        self.stats.accesses_vc += 1;

        let mut found = false;
        let mut hot_block_index = -1i64;
        let tag = address >> self.vc.b;
        let ref mut set = self.vc.set;

        for (count, block) in set.into_iter().enumerate() {
            if block.tag == tag {
                found = true;
                hot_block_index = count as i64;
                break;
            }
        }

        if found {
            set.remove(hot_block_index as usize).unwrap();
            self.stats.victim_hits += 1;
        }

        found
    }

    fn search_l2(&mut self, mode: AccessType, address: u64) -> bool {
        self.stats.accesses_l2 += 1;

        let mut found = false;
        let mut hot_block_index = -1i64;
        let index = (address >> self.l2.b) & ((1u64 << self.l2.indexbits) - 1);
        let tag = address >> (self.l2.b + self.l2.indexbits);

        let selected_set = self.l2.sets.get_mut(index as usize);
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
            panic!("Address index out of bounds in L2! Panic!");
        }

        if !found {
            if let AccessType::Read = mode {
                self.stats.read_misses_l2 += 1;
            } else {
                self.stats.write_misses_l2 += 1;
            }
        }

        found
    }

    fn move_to_l2(&mut self, mode: AccessType, address: u64) {
        // Todo tomorrow morning :3
    }
}
