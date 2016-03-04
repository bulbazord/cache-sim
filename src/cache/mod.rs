// put actual stuff here

pub use self::cachedefs::{AccessType, CacheStats, Cache};

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
