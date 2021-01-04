//! The `ASS.Section` namespace.

use enum_dispatch::enum_dispatch;

/// The four varieties of section.
/// In Lua, this is represented by using the actual class object.
/// For instance, `ASS::Section::SectionType::Drawing` is just `ASS.Section.Drawing`.
pub enum SectionType {
    Comment,
    Drawing,
    Tag,
    Text,
}

/// Marker trait for sections.
/// May be given shared methods later.
#[enum_dispatch]
pub trait SectionTrait {}

/// A section of any variety.
/// In Lua, there is no wrapper layer;
/// a value is simply one of the four section types.
#[enum_dispatch(SectionTrait)]
pub enum Any {
    Comment(Comment),
    Drawing(Drawing),
    Tag(Tag),
    Text(Text),
}

pub struct Comment;
impl SectionTrait for Comment {}

pub struct Drawing;
impl SectionTrait for Drawing {}

mod tag;
pub use tag::TagSection as Tag;
pub use tag::{
    Construction as TagBlockConstruction, Deletion as TagBlockDeletion,
    Insertion as TagBlockInsertion,
};

pub struct Text;
impl SectionTrait for Text {}
