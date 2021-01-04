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
pub type DrawingBase = Todo;
pub type YutilsFont = Todo;

/// Represents the standard line object that any auto4 script would work with.
///
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

pub type StyleName = Option<Either<String, Tag::Reset>>;
pub type CleanLevel = u8;

/// The width, height, descent, and external leading of a piece of text,
/// as returned by `aegisub.text_extents.
///
/// Returned by various `getTextExtents` functions as four values, not a list.
pub struct TextExtents(
    // width
    pub f64,
    // height
    pub f64,
    // descent
    pub f64,
    // ext_lead
    pub f64,
);

/// The bounding box of a piece of text, as returned by `Yutils.shape_bounding`.
///
/// The `x1`, `y1`, `x2`, and `y2` fields are accessed using numerical indices in Lua.
pub struct Bounds {
    pub v_0: f64,
    pub v_1: f64,
    pub v_2: f64,
    pub v_3: f64,
    pub w: f64,
    pub h: f64,
}

/// The metrics of a font, as returned by `yutils_font_handle.metrics`.
///
/// `bounds` is present iff `getTextMetrics` was called with `calculateBounds = true`.
pub struct Metrics {
    pub ascent: f64,
    pub descent: f64,
    pub internal_leading: f64,
    pub external_leading: f64,
    pub height: f64,
    pub width: f64,
    pub bounds: Option<Bounds>,
}

#[path = "LineContents.rs"]
mod line_contents;
pub use line_contents::LineContents;

fn parse<'l>(line: &'l mut Line) -> Todo { stub!() }
