/// This function should be called in each test (at least any test that requires logging or
/// would like deterministic results from RNG). It initializes the [`env_logger`] (with test
/// settings) and seeds the [`fastrand`] RNG with the number 0.
crate fn crate_before_test() {
    if let Err(e) = env_logger::builder().is_test(true).try_init() {
        eprintln!("Failed to initialize env_logger: {}", e);
    };
    crate::util::random::init_rng_seeded(0);
}

/// Checks whether all elements in the first collection are also in the second.
#[macro_export]
macro_rules! assert_contains_all {
    ($needle:expr, $haystack:expr) => {
        for n in $needle.iter() {
            assert!($haystack.contains(n))
        }
    };
}

/// Macro to check if the contents of the first argument are equal to the contents
/// of the second, regardless of the order.
#[macro_export]
macro_rules! assert_unordered_match {
    ($needle:expr, $haystack:expr) => {
        assert_eq!(
            $needle.len(),
            $haystack.len(),
            "The collections have different lengths."
        );
        for n in $needle.iter() {
            assert!($haystack.contains(n));
        }
    };
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_contains_all_works() {
        let haystack = vec!["a", "b", "c", "d"];
        let needle = vec!["a", "c"];

        assert_contains_all!(needle, haystack);
    }
}
