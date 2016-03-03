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
pub struct Cache {
    pub c:                  u64,
    pub b:                  u64,
    pub s:                  u64,
    pub indexbits:          u64,
    pub max_blocks_per_set: u64,
    pub num_of_sets:        u64,
}

//TODO: ADD STORAGE USING VECDEQUE
impl Cache {
    pub fn new(c: u64, b: u64, s: u64) -> Cache {
        Cache {
            c: c,
            b: b,
            s: s,
            indexbits: c - b - s,
            max_blocks_per_set: (1u64 << s),
            num_of_sets: (1u64 << (c - b - s)),
        }
    }
}

#[derive(Debug)]
pub enum AccessType {
    Read,
    Write,
}
