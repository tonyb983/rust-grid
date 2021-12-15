use crate::logging::trace;

fn get_random_seed() -> u64 {
    trace!("get_random_seed");
    let elapsed = std::time::SystemTime::UNIX_EPOCH.elapsed().expect("get_random_seed - Could not get elapsed time from UNIX EPOCH");
    let nanos = elapsed.as_nanos();
    match u64::try_from(nanos) {
        Ok(seed) => seed,
        Err(err) => {
            trace!("get_random_seed - Could not convert elapsed time to u64, using as_secs instead. Error = {:?}", err.to_string());
            elapsed.as_secs()
        }
    }
}

/// ## [`init_rng()`]
/// Initializes the [`fastrand`] RNG with a *"random"-ish* seed determined by the current time.
/// 
/// ## Panics
/// This function will panic if the system clock cannot be read.
pub fn init_rng() {
    trace!("init_rng");
    let seed = get_random_seed();
    trace!("init_rng - seed = {}", seed);
    fastrand::seed(seed);
}

/// Initializes the [`fastrand`] RNG with the given u64 [`seed`].
pub fn init_rng_seeded(seed: u64) {
    trace!("init_rng_seeded");
    fastrand::seed(seed);
}