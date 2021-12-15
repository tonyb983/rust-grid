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
