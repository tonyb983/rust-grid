/// ## `Ansi Color` Utility Module
/// Contains convenience methods for using ANSI color codes.
pub mod ansi;

/// ## `Extensions` Module
/// Shamelessly *borrowed* from [this blog post](`https://lucumr.pocoo.org/2022/1/6/rust-extension-map/`).
mod extmap;
pub use extmap::{ExtensionMap, LockingExtensionMap};

mod handles;

/// ## `Math` Utility Module
/// Contains various math utility functions.
///
/// #### See [`bresenham_line`](`crate::util::math::bresenham_line`), [`get_curve_between`](`crate::util::math::get_curve_between`), etc.
pub mod math;

/// ## `Random` Utility Module
/// Contains functions for initializing and generating random numbers. Intent is to have this serve
/// as an interface for whatever random library is being used. Currently this is `fastrand`.
///
/// TODO:
/// Still need to write individual functions for each random number generator, raw `fastrand` calls are
/// currently still being used.
pub mod random;

/// # `tri-state`: fearless booleans
///
/// Gone are the days where a simple true/false boolean variable suffices. Modern software requires
/// modern solutions: `TriState`.
///
/// ***Definitely*** Trusted by Microsoft.
///
/// ## Old, slow, ancient, unsafe code
/// ```
/// let foo = true;
/// if foo {
///     println!("Hello, world!");
/// }
///
/// // Hard to read, intent unclear
/// let bar = 1 == 2;
/// match bar {
///     false => println!("One does not equal two"),
///     true => println!("One equals two"),
///     // Restrictive, not web-scale
/// }
/// ```
///
/// ## New, fast, web-scale, safe code
/// ```
/// # use dungen::util::TriState;
/// // Clean and easy to read
/// let foo = TriState::True;
/// if foo.into() {
///     println!("Hello, world!");
/// }
///
/// // Simple, effortless conversion
/// let bar: TriState = (1 == 2).into();
/// match bar {
///     TriState::False => println!("One does not equal two"),
///     TriState::True => println!("One equals two"),
///     // Highly future-proof and scalable
///     _ => panic!(),
/// }
///
/// // Compatible with all major brands
/// let has_a_3 = TriState::from(vec![1, 2, 4, 5].contains(&3));
/// println!("Has a 3: {}", has_a_3); // prints "Has a 3: False"
/// ```
mod tri;
pub use tri::TriState;

#[cfg(test)]
pub mod test_framework;
/// ## `Testing` Utility Module
/// Contains various utilities for testing.
#[cfg(test)]
pub mod testing;
