#[derive(Debug)]
pub struct CacheStats {
    pub accesses:           u64,
    pub accesses_l2:        u64,
    pub accesses_vc:        u64,
    pub reads:              u64,
    pub read_misses_l1:     u64,
    pub read_misses_l2:     u64,
    pub writes:             u64,
    pub write_misses_l1:    u64,
    pub write_misses_l2:    u64,
    pub write_back_l1:      u64,
    pub write_back_l2:      u64,
    pub victim_hits:        u64,
    pub avg_access_time_l1: f64,
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
}

#[derive(Debug)]
pub enum AccessType {
    Read,
    Write,
}
