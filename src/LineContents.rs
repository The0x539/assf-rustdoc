use either::Either;
use std::collections::HashMap as Map;

use crate::{
    Callback, CleanLevel, Line, LineBounds, Metrics, RunCount,
    Section::{self, Any as AnySection, SectionType},
    Style, StyleName,
    Tag::{self, Any as AnyTag},
    TagList, TextExtents, Todo,
};

pub struct LineContents<'l> {
    line: &'l mut Line,
    sections: Vec<AnySection>,
}

impl<'l> LineContents<'l> {
    pub fn new(
        line: &'l mut Line,
        sections: Option<Vec<AnySection>>,
        copyAndCheckSections: bool,
    ) -> Self {
        stub!()
    }

    pub fn updateRefs(&mut self) -> bool { stub!() }

    pub fn getString<A1, A2, A3, Pr>(
        &self,
        includeEmptySections: Option<bool>,
        currDrawingState: Option<Tag::Drawing>,
        predicate: Option<Either<Pr, &[SectionType]>>,
        predicateLookAhead: Option<bool>,
        a1: A1,
        a2: A2,
        a3: A3,
    ) -> String
    where
        Pr: FnMut(
            &AnySection,
            usize,
            &[AnySection],
            SectionType,
            &mut A1,
            &mut A2,
            &mut A3,
        ) -> bool,
    {
        stub!()
    }

    pub fn get(
        &mut self,
        sectionClasses: Option<&[SectionType]>,
        start: Option<usize>,
        end_: Option<usize>,
        relative: Option<bool>,
    ) -> Vec<&mut AnySection> {
        stub!()
    }

    pub fn callback(
        &mut self,
        callback: impl Callback<AnySection>,
        sectionClasses: Option<&[SectionType]>,
        start: Option<usize>,
        end_: Option<usize>,
        relative: Option<bool>,
        reverse: Option<bool>,
    ) -> RunCount {
        stub!()
    }

    /// Returns the sections that were inserted.
    pub fn insertSections(
        &mut self,
        sections: Either<AnySection, Vec<AnySection>>,
        index: Option<usize>,
    ) -> Vec<&mut AnySection> {
        stub!()
    }

    /// Statically, this method has four signatures.
    /// * `(&mut self)` removes all sections.
    /// * `(&mut self, usize, Option<usize>)` removes a range of sections.
    /// * `(&mut self, &Section)` removes a specific section.
    /// * `(&mut self, &[&Section])` removes a set of specific sections.
    ///
    /// In all cases, returns a list of the sections that were removed.
    pub fn removeSections<T>(&mut self, start: T, end_: Option<T>) -> Vec<AnySection> { stub!() }

    pub fn modTags(
        &mut self,
        tagNames: &[&str],
        callback: impl Callback<AnyTag>,
        start: Option<usize>,
        end_: Option<usize>,
        relative: Option<bool>,
    ) -> RunCount {
        stub!()
    }

    pub fn getTags(
        &mut self,
        tagNames: &[&str],
        start: Option<usize>,
        end_: Option<usize>,
        relative: Option<bool>,
    ) -> Vec<AnyTag> {
        stub!()
    }

    /// The allowed types for `tagList` aren't directly inherited from
    /// [Section::Tag::insertTags], but they happen to be the same.
    pub fn replaceTags(
        &mut self,
        tagList: impl Section::TagBlockInsertion,
        start: Option<usize>,
        end_: Option<usize>,
        relative: Option<bool>,
        insertRemaining: Option<bool>,
    ) {
        stub!()
    }

    pub fn removeTags(
        &mut self,
        tags: impl Section::TagBlockDeletion,
        start: Option<usize>,
        end_: Option<usize>,
        relative: Option<bool>,
    ) -> Vec<AnyTag> {
        stub!()
    }

    pub fn insertTags(
        &mut self,
        tags: impl Section::TagBlockInsertion,
        index: Option<usize>,
        sectionPosition: Option<usize>,
        direct: Option<bool>,
    ) -> &mut Vec<AnyTag> {
        stub!()
    }

    pub fn insertDefaultTags(
        &mut self,
        tagNames: Either<&str, &[&str]>,
        index: Option<usize>,
        sectionPosition: Option<usize>,
        direct: Option<bool>,
    ) -> Either<&mut AnyTag, &mut Vec<AnyTag>> {
        stub!()
    }

    pub fn getEffectiveTags(
        &self,
        index: Option<usize>,
        includeDefault: Option<bool>,
        includePrevious: Option<bool>,
        copyTags: Option<bool>,
    ) -> TagList {
        stub!()
    }

    pub fn getTagCount(&self) -> usize { stub!() }
    pub fn stripTags(&mut self) -> &mut Self { stub!() }
    pub fn stripText(&mut self) -> &mut Self { stub!() }
    pub fn stripComments(&mut self) -> &mut Self { stub!() }
    pub fn stripDrawings(&mut self) -> &mut Self { stub!() }

    pub fn commit(
        &mut self,
        line: Option<&mut Line>,
        includeEmptySections: Option<bool>,
        text: Option<&str>,
    ) -> String {
        stub!()
    }

    pub fn undoCommit(&mut self, line: Option<&mut Line>) -> bool { stub!() }

    pub fn cleanTags(
        &mut self,
        level: Option<CleanLevel>,
        mergeConsecutiveSections: Option<bool>,
        defaultToKeep: Option<bool>,
        tagSortOrder: Option<&[i64]>,
    ) -> &mut Self {
        stub!()
    }

    pub fn splitAtTags(
        &mut self,
        cleanLevel: Option<CleanLevel>,
        reposition: Option<bool>,
        writeOrigin: Option<bool>,
    ) -> Vec<Line> {
        stub!()
    }

    pub fn splitAtIntervals(
        &mut self,
        callback: Either<usize, impl FnMut(usize, usize) -> usize>,
        cleanLevel: Option<CleanLevel>,
        reposition: Option<bool>,
        writeOrigin: Option<bool>,
    ) -> Vec<Line> {
        stub!()
    }

    pub fn splitAtIndexes(
        &mut self,
        indices: Either<usize, &[usize]>,
        cleanLevel: Option<CleanLevel>,
        reposition: Option<bool>,
        writeOrigin: Option<bool>,
    ) -> Vec<Line> {
        stub!()
    }

    pub fn repositionSplitLines(
        &mut self,
        splitLines: &mut [Line],
        writeOrigin: Option<bool>,
    ) -> &mut [Line] {
        stub!()
    }

    pub fn trim(&mut self) { stub!() }

    pub fn getStyleRef(&mut self, style: StyleName) -> &mut Style { stub!() }

    pub fn getPosition(
        &self,
        style: StyleName,
        align: Option<Either<Tag::Alignment, Tag::Align>>,
        forceDefault: Option<bool>,
    ) -> (Tag::Position, Tag::Align, Tag::Origin) {
        stub!()
    }

    pub fn getDefaultTags(
        &self,
        style: StyleName,
        copyTags: Option<bool>,
        useOvrAlign: Option<bool>,
    ) -> TagList {
        stub!()
    }

    /// The `coerce` argument is passed to [Section::Text::getTextExtents],
    /// but that function accepts no such argument.
    ///
    /// I imagine the intended type might be an (optional) boolean,
    /// but at the time of writing, the arg is effectively unused.
    pub fn getTextExtents(&self, coerce: ()) -> TextExtents { stub!() }

    pub fn getLineBounds(&self, noCommit: Option<bool>, keepRawText: Option<bool>) -> LineBounds {
        stub!()
    }

    pub fn getTextMetrics(&self, calculateBounds: bool, coerce: ()) -> Metrics { stub!() }

    pub fn getSectionCount(
        &self,
        classes: Option<&[SectionType]>,
    ) -> Either<usize, (Map<SectionType, usize>, usize)> {
        stub!()
    }

    pub fn isAnimated(&self) -> bool { stub!() }

    pub fn reverse(&mut self) -> &mut Self { stub!() }
}

fn parse<'l>(line: &'l mut Line) -> Todo { stub!() }
