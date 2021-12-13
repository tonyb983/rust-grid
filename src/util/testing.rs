/// Example Use:
/// ```
/// #[test]
/// fn test_something_interesting() {
///     run_tests(|| {
///         let true_or_false = do_the_test();
///
///         assert!(true_or_false);
///     })
/// }
/// ```
///
pub fn run_tests(setup: fn(), teardown: fn(), tests: Vec<fn()>) {
    setup();

    let mut results = Vec::new();

    for t in tests {
        let result = std::panic::catch_unwind(t);
        results.push(result);
    }

    teardown();

    for result in results {
        assert!(result.is_ok());
    }
}

/// This function should be called in each test (at least any test that requires logging)
/// or the [`fastrand`] crate. It initializes the [`env_logger`] (with test settings) and
/// seeds the [`fastrand`] RNG with the number 0.
pub fn before_test() {
    let _ = env_logger::builder().is_test(true).try_init();
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
