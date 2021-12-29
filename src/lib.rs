//! # Dungen
//!
//! My experimentation with all things procedurally generated, pathfinding, 2D grid / matrix operations, etc.
//!
//! Eventual goal is to have a library that creates "dungeons" for "games", and any other functionality
//! I decide to "implement", whatever those words mean to me at the moment.

// TODO At some point I should probably update this with the features I'm actually using.
#![feature(
    associated_type_defaults,
    backtrace,
    inline_const,
    concat_idents,
    crate_visibility_modifier,
    default_free_fn,
    exclusive_range_pattern,
    half_open_range_patterns,
    let_else,
    once_cell,
    test,
    try_blocks
)]
// Activate ALL THE WARNINGS. I want clippy to be as absolutely annoying as fucking possible.
#![warn(
    clippy::pedantic,
    clippy::all,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    rustdoc::all
)]
// Justifications:
// - `clippy::module_name_repititions` - This is maybe something I can be better about but for now it's okay in my book.
// - `clippy::semicolon_if_nothing_returned` - This interferes with the `let_else` feature syntax.
// - `clippy::similar_names` - This interferes with functions that use a lot of intermediate variables (usually for debugging).
// #![allow(
//     clippy::module_name_repetitions,
//     clippy::semicolon_if_nothing_returned,
//     clippy::similar_names
// )]

/// ## `Data` Module
/// The main data types for the library.
///
/// #### See [`crate::data::MapGrid`], [`crate::data::AsPos`], [`crate::data::GridIndex`], [`crate::data::GridPos`], [`crate::data::GridSize`], [`crate::data::GridSquare`], [`crate::data::pos`], [`crate::data::size`], [`crate::data::square`]
pub mod data;

/// ## `Drawing` Module
/// This crate implements various drawing functionality.
///
/// #### See [`crate::draw::Artist`]
pub mod draw;

/// ## `Generation` Module
/// This crate implements various generational algorithms and utilities.
///
/// #### See [`crate::gen::CellularAutomata`]
pub mod gen;

/// ## `Pathfinding` Module
///
/// This crate implements various pathfinding algorithms and utilities.
///
/// #### See [`crate::pf::Pathfinding`]
pub mod pf;

/// ## `Pipeline` Module
/// This crate was a quick experiment in writing a data processing pipeline. Very incomplete.
pub mod pipe;

/// ## `Utility` Module
/// This crate has various utility functions.
///
/// #### See [`init_rng`][`crate::util::random::init_rng`], [`init_rng_seeded`][`crate::util::random::init_rng_seeded`]
/// #### See [`get_curve_between`][`crate::util::math::get_curve_between`], [`absdiff`][`crate::util::math::absdiff`], [`uint_sqrt`][`crate::util::math::uint_sqrt`], [`bresenham_line`][`crate::util::math::bresenham_line`], [`on_same_column`][`crate::util::math::on_same_column`]
pub mod util;

/// ## `Logging` Module
/// This crate (at least for now) simply re-exports the `log` crate.
#[allow(unused_imports)]
crate mod logging {
    pub(crate) use log::{debug, error, info, trace, warn};
}

/// Fake main to run from `./bin/runner.rs`.
pub mod term_menu {
    enum ExampleLabels {
        List,
        Scroll,
        EmptyString,
        NonEmptyString,
        Number,
    }

    impl From<ExampleLabels> for String {
        fn from(val: ExampleLabels) -> Self {
            match val {
                ExampleLabels::List => "list".to_string(),
                ExampleLabels::Scroll => "scroll".to_string(),
                ExampleLabels::EmptyString => "estr".to_string(),
                ExampleLabels::NonEmptyString => "nestr".to_string(),
                ExampleLabels::Number => "num".to_string(),
            }
        }
    }

    impl From<ExampleLabels> for &str {
        fn from(val: ExampleLabels) -> Self {
            match val {
                ExampleLabels::List => "list",
                ExampleLabels::Scroll => "scroll",
                ExampleLabels::EmptyString => "estr",
                ExampleLabels::NonEmptyString => "nestr",
                ExampleLabels::Number => "num",
            }
        }
    }

    /// Fake main for `terminal_menu` `basic` example.
    pub fn run_simple() {
        use terminal_menu::{button, label, menu, mut_menu, run};
        let menu = menu(vec![
            // label:
            //  not selectable, usefule as a title, separator, etc...
            label("----------------------"),
            label("terminal-menu"),
            label("use wasd or arrow keys"),
            label("enter to select"),
            label("'q' or esc to exit"),
            label("-----------------------"),
            // button:
            //  exit the menu
            button("Alice"),
            button("Bob"),
            button("Charlie"),
        ]);
        run(&menu);

        // you can get the selected buttons name like so:
        println!("Selected: {}", mut_menu(&menu).selected_item_name());
    }

    /// Fake main for `terminal_menu` `selection` example.
    pub fn run_select() {
        use terminal_menu::{button, label, list, menu, mut_menu, run, scroll};
        let menu = menu(vec![
            label("lists and scrolls"),
            // with list and scroll you can select a value from a group of values
            // you can change the selected value with arrow keys, wasd, or enter

            // use arrow keys or wasd
            // enter to select

            // list:
            //  show all values
            //  surround the selected value with brackets
            list(ExampleLabels::List, vec!["Alice", "Bob", "Charlie"]),
            // scroll:
            //  show only the selected item
            scroll(ExampleLabels::Scroll, vec!["Alice", "Bob", "Charlie"]),
            button("exit"),
        ]);
        run(&menu);
        {
            let mm = mut_menu(&menu);
            println!("{}", mm.selection_value(ExampleLabels::List.into()));
            println!("{}", mm.selection_value(ExampleLabels::Scroll.into()));
        }
    }

    /// Fake main for `terminal_menu` `long` example.
    pub fn run_long() {
        use terminal_menu::{button, menu, mut_menu, run};
        let menu = menu(
            // create buttons representing numbers from 1 to 100
            (1..100).map(|n| button(format!("{}", n))).collect(),
        );
        run(&menu);
        println!("{}", mut_menu(&menu).selected_item_name());
    }

    /// Fake main for `terminal_menu` `strings and numerics` example.
    pub fn run_strnum() {
        use terminal_menu::{button, label, menu, mut_menu, numeric, run, string};
        let menu = menu(vec![
            label("strings and numerics"),
            // string:
            //  a string of characters
            //  the last arguments specifies if empty strings are allowed

            // empty strings allowed:
            string(ExampleLabels::EmptyString, "empty allowed", true),
            // empty strings not allowed:
            string(ExampleLabels::NonEmptyString, "cannot be empty", false),
            // numeric:
            //  a floating point number
            numeric(
                ExampleLabels::Number,
                // default
                4.5,
                // step
                Some(1.5),
                // minimum
                None,
                // maximum
                Some(150.0),
            ),
            button("exit"),
        ]);
        run(&menu);
        {
            let mm = mut_menu(&menu);
            println!("{}", mm.selection_value(ExampleLabels::EmptyString.into()));
            println!(
                "{}",
                mm.selection_value(ExampleLabels::NonEmptyString.into())
            );
            println!("{}", mm.numeric_value(ExampleLabels::Number.into()));
        }
    }
}

pub mod ansi_col {
    pub fn run_basic() {
        println!("{}Colored {}{}text {}{}hopefully!{} (except this)", "\x1b[4;32m", "\x1b[0m", "\x1b[34m", "\x1b[0m", "\x1b[21m", "\x1b[0m");
    }
}
