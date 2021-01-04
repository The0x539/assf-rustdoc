use crate::{DrawingBase, Metrics, Style, StyleName, TagList, TextExtents, YutilsFont};

/// The `ASS.Section.Text` class.
///
/// Represents, well, a piece of text, the part of a line that gets displayed.
///
/// Contains a [String].
pub struct TextSection {
    pub value: String,
}

impl super::SectionTrait for TextSection {}

impl TextSection {
    pub fn new(value: String) -> Self { stub!() }

    pub fn getString(&self) -> &str { stub!() }

    pub fn getEffectiveTags(
        &self,
        includeDefault: Option<bool>,
        includePrevious: Option<bool>,
        copyTags: Option<bool>,
    ) -> TagList {
        stub!()
    }

    pub fn getStyleTable(&self, name: StyleName) -> &mut Style { stub!() }

    pub fn getTextExtents(&self) -> TextExtents { stub!() }

    pub fn getTextMetrics(&self, calculateBounds: Option<bool>) -> Metrics { stub!() }

    pub fn getShape(&self, applyRotation: Option<bool>) -> DrawingBase { stub!() }

    /// Replaces `self` with a [Drawing](super::Drawing) section in the line that contains it.
    ///
    /// Kinda tricky to represent this in Rust.
    /// Hopefully you get the idea.
    pub fn convertToDrawing(self, applyRotation: Option<bool>) -> super::Drawing { stub!() }

    /// Invokes [convertToDrawing], then [super::Drawing::expand].
    pub fn expand(self, x: Option<f64>, y: Option<f64>) -> super::Drawing { stub!() }

    pub fn getYutilsFont(&self) -> (YutilsFont, TagList) { stub!() }

    pub fn splitAtChar(&mut self, index: usize, mutate: bool) -> (Self, Self) { stub!() }

    #[doc(hidden)]
    /// Unfinished function
    pub fn insertTagsAtChar(&mut self, index: usize, tags: super::Tag) { stub!() }

    pub fn trimLeft(&mut self) {}
    pub fn trimRight(&mut self) {}
    pub fn trim(&mut self) {}
}
