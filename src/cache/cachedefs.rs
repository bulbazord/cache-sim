/*#[derive(Debug)]
pub struct Cache {
}*/

#[derive(Debug)]
pub struct CacheStats {
    accesses:           u64,
    accesses_l2:        u64,
    accesses_vc:        u64,
    reads:              u64,
    read_misses_l1:     u64,
    read_misses_l2:     u64,
    writes:             u64,
    write_misses_l1:    u64,
    write_misses_l2:    u64,
    write_back_l1:      u64,
    write_back_l2:      u64,
    victim_hits:        u64,
    avg_access_time_l1: f64,
}

impl CacheStats {
    pub fn new() -> CacheStats {
        CacheStats {
            accesses:           0u64,
            accesses_l2:        0u64,
            accesses_vc:        0u64,
            reads:              0u64,
            read_misses_l1:     0u64,
            read_misses_l2:     0u64,
            writes:             0u64,
            write_misses_l1:    0u64,
            write_misses_l2:    0u64,
            write_back_l1:      0u64,
            write_back_l2:      0u64,
            victim_hits:        0u64,
            avg_access_time_l1: 0f64,
        }
    }

    pub fn print_statistics(&self) {
        println!("Cache Statistics");
        println!("Accesses: {}", self.accesses);
        println!("Accesses to L2: {}", self.accesses_l2);
        println!("Accesses to VC: {}", self.accesses_vc);
        println!("Reads: {}", self.reads);
        println!("Read misses to L1: {}", self.read_misses_l1);
        println!("Read misses to L2: {}", self.read_misses_l2);
        println!("Writes: {}", self.writes);
        println!("Write misses to L1: {}", self.write_misses_l1);
        println!("Write misses to L2: {}", self.write_misses_l2);
        println!("Write backs from L1: {}", self.write_back_l1);
        println!("Write backs from L2: {}", self.write_back_l2);
        println!("Victim hits: {}", self.victim_hits);
        println!("Average access time: {}", self.avg_access_time_l1);
    }
}
