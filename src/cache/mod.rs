// put actual stuff here

pub use self::cachedefs::CacheStats;
pub use self::cachedefs::AccessType;

pub mod cachedefs;

#[derive(Debug)]
pub struct Cache {
    pub stats: CacheStats,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            stats: CacheStats::new(),
        }
    }

    pub fn setup_cache(c1: u64, b1: u64, s1: u64, v: u64, c2: u64, b2: u64, s2: u64)
        -> Cache {
        println!("Time to set up the cache");
        println!("Cache params: ({}, {}, {}, {}, {}, {}, {})", c1, b1, s1, v, c2, b2, s2);
        Cache::new()
    }

    pub fn cache_access(&mut self, mode: AccessType, address: u64) {
        println!("Trying to {:?} at address {:#X}", mode, address);
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
        println!("Victim hits: {}", self.stats.victim_hits);
        println!("Average access time: {}", self.stats.avg_access_time_l1);
    }
}
