#![allow(dead_code, unused_variables)]
#![allow(non_camel_case_types, non_snake_case)]

use core::num::NonZeroUsize;
use either::Either;

#[rustfmt::skip]
macro_rules! stub { () => { unimplemented!() }; }

pub enum Todo {}
pub type TagList = Todo;
pub type Style = Todo;
pub type LineBounds = Todo;

/// Represents the standard line object that any auto4 script would work with.
/// Also represents the `a-mo.Line` class, which is supposed to be similar.
///
/// Dynamic typing is such a headache.
pub struct Line(String);

/// A return type used for many callback-based functions.
/// Corresponds to expressions of the form `n > 0 and n or false` in Lua.
pub type RunCount = Option<NonZeroUsize>;

pub mod Parser {
    pub mod LineText {
        use crate::{Line, LineContents, Section};

        fn getSections(line: &mut Line) -> Vec<Section::Any> { stub!() }

        fn getLineContents<'l>(line: &'l mut Line) -> LineContents<'l> { stub!() }
    }
}

/// A callback, used in one of this library's several `callback` methods.
///
/// Statically, a callback should generally match one of this trait's implementors.
///
/// Dynamically, the return type behavior is:
/// * returning `false` for an entry will delete it.
/// * returning `nil` or `true` for an entry will retain it.
/// * returning any other value for an entry will replace it with the returned value.
pub trait Callback<T> {}

/// "Filter" style callbacks.
impl<T> Callback<T> for fn(&T, &[T], usize, usize) -> bool {}
/// "Map" style callbacks.
impl<T> Callback<T> for fn(T, &[T], usize, usize) -> T {}
/// "For each" style callbacks.
impl<T> Callback<T> for fn(&mut T, &[T], usize, usize) {}

pub mod Tag;

pub mod Section;
pub use Section::SectionType;

pub type StyleName = Option<Either<String, Tag::Reset>>;
pub type CleanLevel = u8;
pub struct TextExtents(
    /// width
    f64,
    /// height
    f64,
    /// descent
    f64,
    /** ext_lead */ f64,
);
pub struct Bounds {
    _0: f64,
    _1: f64,
    _2: f64,
    _3: f64,
    w: f64,
    h: f64,
}
pub struct Metrics {
    ascent: f64,
    descent: f64,
    internal_leading: f64,
    external_leading: f64,
    height: f64,
    bounds: Option<Bounds>,
}

#[path = "LineContents.rs"]
mod line_contents;
pub use line_contents::LineContents;

fn parse<'l>(line: &'l mut Line) -> Todo { stub!() }
