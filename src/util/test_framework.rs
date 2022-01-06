//! This module is a potential basis for a test framework / harness. Some skeletal framework code is implemented in
//! [TestModule] and [TestUnit].
//! Currently nothing uses or tests this code, and it is very much a work in progress
//! (and is likely as buggy as those words usually suggest).
//!
//! Here is an idea of how these pieces would fit together to form a rust test framework. Some of this was inspired /
//! borrowed / stolen from [this great blog post] on creating a custom test harness in rust.
//!
//! [TestUnit] - This is a single unit test. It is a single test function, an associated name (ideally I'd like to have this be pulled automatically from the function name, which I believe can only be done via macro in the current state of rust "reflection" type capability), an optional "before test" function, an optional "after test" function, and a boolean flag to indicate whether or not the test should panic. Future additions should include a collection of tags associated with the test, and maybe a flag to indicate whether or not the test should be run. Combined these would allow for fine grain control over which tests are run.
//!
//! [TestModule] - This is a collection of [TestUnit]s, ideally centered around one module or one functionality. This would be a [Vec] or other collection type containing the individual tests, an optional "before all tests" function, and an optional "after all tests" function.
//!
//! A crate such as [inventory] could be used to collect all of the [TestModule]s into a single "collection" to be run.
//!
//! [TestUnit]: crate::util::test_framework::TestUnit
//! [TestModule]: crate::util::test_framework::TestModule
//! [inventory]: https://crates.io/crates/inventory
//! [this great blog post]: https://www.infinyon.com/blog/2021/04/rust-custom-test-harness
#![allow(clippy::doc_markdown, dead_code, unused)]

/// A simple test runner function.
/// Steps:
/// - Run `setup` function.
/// - Run each test, saving the result from each in an array.
/// - Run the `teardown` function.
/// - Assert each that each result `is_ok`.
///
/// ### Panics
/// - Function panics if any result is `Err`.
///
/// ### Example Use:
/// ```ignore
/// // Note the lack of #[test] attribute, we don't want these functions to be run automatically.
/// fn some_test() {
///     let something = vec![1, 2, 3];
///     assert_eq!(something.len(), 3);
/// }
///
/// fn some_other_test() {
///     let something = vec![1, 2, 3];
///     // Uh oh!
///     assert_eq!(something.len(), 4);
/// }
///
/// fn before_all_tests() {
///     // Set up logging, seed rng, etc. etc.
/// }
///
/// fn after_all_tests() {
///     // Write log to file, close some resource, etc. etc.
/// }
///
/// // This function gets the test attribute, which will in turn call the above tests and assert on the result.
/// #[test]
/// fn test_my_module() {
///     run_tests(
///         before_all_tests,
///         after_all_tests,
///         vec![some_test, some_other_test],
///     );
/// }
/// ```
///
crate fn run_tests(setup: fn(), teardown: fn(), tests: Vec<fn()>) {
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

crate struct TestUnit {
    name: &'static str,
    test: fn(),
    should_panic: bool,
    before: Option<fn()>,
    after: Option<fn()>,
}

impl TestUnit {
    crate fn basic(name: &'static str, test: fn()) -> TestUnit {
        TestUnit {
            name,
            test,
            should_panic: false,
            before: None,
            after: None,
        }
    }

    crate fn should_panic(
        name: &'static str,
        test: fn(),
        before: Option<fn()>,
        after: Option<fn()>,
    ) -> TestUnit {
        TestUnit {
            name,
            test,
            should_panic: true,
            before,
            after,
        }
    }

    crate fn with_before(name: &'static str, test: fn(), before: fn()) -> TestUnit {
        TestUnit {
            name,
            test,
            should_panic: false,
            before: Some(before),
            after: None,
        }
    }

    crate fn with_after(name: &'static str, test: fn(), after: fn()) -> TestUnit {
        TestUnit {
            name,
            test,
            should_panic: false,
            before: None,
            after: Some(after),
        }
    }

    crate fn full(
        name: &'static str,
        test: fn(),
        should_panic: bool,
        before: Option<fn()>,
        after: Option<fn()>,
    ) -> Self {
        Self {
            name,
            test,
            should_panic,
            before,
            after,
        }
    }
}

crate struct TestModule {
    setup: fn(),
    teardown: fn(),
    tests: Vec<TestUnit>,
}

impl TestModule {
    crate fn new<T: Into<TestUnit>>(
        setup: fn(),
        teardown: fn(),
        each_test: impl Iterator<Item = T>,
    ) -> Self {
        let mut tests = Vec::new();
        for test in each_test {
            let test = test.into();
            tests.push(test);
        }
        Self {
            setup,
            teardown,
            tests,
        }
    }

    crate fn run(&self) {
        (self.setup)();

        let mut results = Vec::new();

        for t in &self.tests {
            if let Some(before) = t.before {
                before();
            }

            let result = std::panic::catch_unwind(t.test);
            results.push((result, t.should_panic));

            if let Some(after) = t.after {
                after();
            }
        }

        (self.teardown)();

        for (result, sp) in results {
            if sp {
                assert!(result.is_err());
            } else {
                assert!(result.is_ok());
            }
        }
    }
}
