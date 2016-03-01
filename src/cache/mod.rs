// put actual stuff here

pub use self::cachedefs::Cache;
pub use self::cachedefs::AccessType;

pub mod cachedefs;

pub fn setup_cache(c1: u64, b1: u64, s1: u64, v: u64, c2: u64, b2: u64, s2: u64) 
    -> Cache {
    println!("Time to set up the cache");
    Cache::new()
}

pub fn cache_access(acctype: AccessType, address: u64, cache: &mut Cache) {
    println!("Modifying cache and access stuff");
}

pub fn complete_cache(cache: &mut Cache) {
    println!("Completing the cache!");
}
