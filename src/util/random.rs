use log::trace;

/// Initializes the **fastrand** RNG with a seed determined by the current time.
pub fn init_rng() {
    trace!("init_rng");
    fastrand::seed(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
}

/// Initializes the **fastrand** RNG with the given [`seed`].
pub fn init_rng_seeded(seed: u64) {
    trace!("init_rng_seeded");
    fastrand::seed(seed);
}