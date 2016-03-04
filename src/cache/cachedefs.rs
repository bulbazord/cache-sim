use std::collections::VecDeque;

/* -CacheStats-
 * Cache Stats are for keeping track of what happens during
 * the cache simulation.
 */
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

/* -Cache-
 * A cache is defined by the parameters (c, b, s).
 * The other data is out of convenience. Calculate once and store.
 */
#[derive(Debug)]
pub struct Cache {
    pub c:                  u64,
    pub b:                  u64,
    pub s:                  u64,
    pub indexbits:          u64,
    pub max_blocks_per_set: u64,
    pub num_of_sets:        u64,
    pub sets:               Vec<VecDeque<CacheBlock>>,
}

impl Cache {
    pub fn new(c: u64, b: u64, s: u64) -> Cache {
        let mut ret_val = Cache {
            c: c,
            b: b,
            s: s,
            indexbits: c - b - s,
            max_blocks_per_set: (1u64 << s),
            num_of_sets: (1u64 << (c - b - s)),
            sets: Vec::with_capacity(1usize << (c - b -s)),
        };

        for _i in 0..ret_val.num_of_sets {
            ret_val.sets.push(VecDeque::with_capacity(ret_val.max_blocks_per_set as usize));
        }

        ret_val
    }
}

/* -CacheBlock-
 * A cacheblock (in hardware) is usually realized by tag and some metadata.
 * I put tag in there for a "Calculate once, use many times" situation.
 * The address is kept so that tag/index can be recalculated when moving a block around.
 */
#[derive(Debug)]
pub struct CacheBlock {
    pub address: u64,
    pub tag: u64,
    pub dirty: bool,
}

impl CacheBlock {
    pub fn new(a: u64, t: u64, d: bool) -> CacheBlock {
        CacheBlock {
            address: a,
            tag: t,
            dirty: d,
        }
    }
}

/* -AccessType-
 * Wow enums are cool! An access is either a read or write. Easy as that.
 */
#[derive(Debug, Copy, Clone)]
pub enum AccessType {
    Read,
    Write,
}
