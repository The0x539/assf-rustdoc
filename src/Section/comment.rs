/// The `ASS.Section.Comment` class.
///
/// Represents, an inline comment.
///
/// Contains a [String].
pub struct CommentSection {
    pub value: String,
}

impl super::SectionTrait for CommentSection {}

impl CommentSection {
    // accepts a borrowed str because braces get stripped on construction
    pub fn new(value: &str) -> Self { stub!() }

    // returns an owned String because braces get stripped on access
    pub fn getString(&self) -> String { stub!() }
}
