use crate::{Callback, RunCount, Tag::Any as Tag, TagList};

use either::Either;

/// Types that can be passed as the first argument to [TagSection::new]:
/// * A [TagList].
/// * `nil`.
/// * A [TagSection].
/// * A `string`
/// * A list containing a single `string`.
/// ([don't look at me, I'm just writing docs](https://github.com/TypesettingTools/ASSFoundation/blob/ba2cace60efc39edfdedce1747b2b68aeff0af01/l0/ASSFoundation/Section/Tag.moon#L66-L67))
/// * A list of [Tag] objects.
pub trait Construction {}
impl Construction for TagList {}
impl Construction for () {}
impl Construction for TagSection {}
impl Construction for &str {}
impl Construction for &[&str] {}
impl Construction for &[Tag] {}

/// Types that can be passed as the first argument to [TagSection::insertTags]:
/// * A list of [Tag] objects.
/// * Another [TagSection] object.
/// * A [TagList] object.
/// * A single [Tag].
pub trait Insertion {}
impl Insertion for &[Tag] {}
impl Insertion for TagSection {}
impl Insertion for TagList {}
impl Insertion for Tag {}

/// Types that can be passed as the first argument to [TagSection::removeTags]:
/// * `nil` (will remove all tags within the specified range).
/// * A `string`.
/// * A [Tag].
/// * A list of either `string`s or [Tag]s.
pub trait Deletion {}
impl Deletion for () {}
impl Deletion for &str {}
impl Deletion for Tag {}
impl Deletion for &[&str] {}
impl Deletion for &[Tag] {}
impl Deletion for &[Either<&str, Tag>] {}

/// The `ASS.Section.Tag` class.
///
/// Represents a single block of override tags within a line.
///
/// Contains a list of [Tag] values.
pub struct TagSection {
    tags: Vec<Tag>,
}
impl super::SectionTrait for TagSection {}
impl TagSection {
    pub fn new(
        tags: impl Construction,
        transformableOnly: Option<bool>,
        tagSortOrder: Option<&[i64]>,
    ) -> Self {
        stub!()
    }

    pub fn getStyleTable(&mut self, todo_finish_this_signature: ()) -> ! { stub!() }

    pub fn callback(
        &mut self,
        callback: impl Callback<Tag>,
        tagNames: Either<&str, &[&str]>,
        first: Option<usize>,
        last: Option<usize>,
        relative: Option<bool>,
        reverse: Option<bool>,
    ) -> RunCount {
        stub!()
    }

    pub fn modTags(
        &mut self,
        tagNames: Either<&str, &[&str]>,
        callback: impl Callback<Tag>,
        first: Option<usize>,
        last: Option<usize>,
        relative: Option<bool>,
    ) -> RunCount {
        stub!()
    }

    pub fn getTags(
        &mut self,
        first: Option<usize>,
        last: Option<usize>,
        relative: Option<bool>,
    ) -> Vec<&mut Tag> {
        stub!()
    }

    /// Remove this tag section from its parent object (if it has one) and return it.
    pub fn remove(self) -> Self { stub!() }

    /// This function can also be called with the first argument omitted, i.e.:
    /// ```
    /// fn removeTags(first, last, relative)
    /// ```
    /// This is equivalent to explicitly passing nil as the first argument
    /// and will delete *all* tags in the specified range.
    pub fn removeTags(
        &mut self,
        tags: impl Deletion,
        first: Option<usize>,
        last: Option<usize>,
        relative: Option<bool>,
    ) -> (Vec<Tag>, usize) {
        stub!()
    }

    /// Returns the section's new contents.
    pub fn insertTags(
        &mut self,
        tags: impl Insertion,
        index: Option<usize>,
    ) -> Either<&mut Tag, &mut Vec<Tag>> {
        stub!()
    }

    pub fn insertDefaultTags(
        &mut self,
        tag_names: Either<&str, &[&str]>,
        index: Option<usize>,
    ) -> Either<&mut Tag, &mut Vec<Tag>> {
        stub!()
    }

    pub fn getString(&self) -> String { stub!() }

    pub fn getEffectiveTags(
        &self,
        includeDefault: Option<bool>,
        includePrevious: Option<bool>,
        copyTags: Option<bool>,
    ) -> TagList {
        stub!()
    }
}
