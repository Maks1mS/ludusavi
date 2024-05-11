//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTextBlockValueType {
        NSTextBlockAbsoluteValueType = 0,
        NSTextBlockPercentageValueType = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTextBlockDimension {
        NSTextBlockWidth = 0,
        NSTextBlockMinimumWidth = 1,
        NSTextBlockMaximumWidth = 2,
        NSTextBlockHeight = 4,
        NSTextBlockMinimumHeight = 5,
        NSTextBlockMaximumHeight = 6,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextBlockLayer {
        NSTextBlockPadding = -1,
        NSTextBlockBorder = 0,
        NSTextBlockMargin = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTextBlockVerticalAlignment {
        NSTextBlockTopAlignment = 0,
        NSTextBlockMiddleAlignment = 1,
        NSTextBlockBottomAlignment = 2,
        NSTextBlockBaselineAlignment = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTextTableLayoutAlgorithm {
        NSTextTableAutomaticLayoutAlgorithm = 0,
        NSTextTableFixedLayoutAlgorithm = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextBlock")]
    pub struct NSTextBlock;

    #[cfg(feature = "AppKit_NSTextBlock")]
    unsafe impl ClassType for NSTextBlock {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTextBlock")]
unsafe impl NSCoding for NSTextBlock {}

#[cfg(feature = "AppKit_NSTextBlock")]
unsafe impl NSCopying for NSTextBlock {}

#[cfg(feature = "AppKit_NSTextBlock")]
unsafe impl NSObjectProtocol for NSTextBlock {}

#[cfg(feature = "AppKit_NSTextBlock")]
unsafe impl NSSecureCoding for NSTextBlock {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextBlock")]
    unsafe impl NSTextBlock {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method(setValue:type:forDimension:)]
        pub unsafe fn setValue_type_forDimension(
            &self,
            val: CGFloat,
            r#type: NSTextBlockValueType,
            dimension: NSTextBlockDimension,
        );

        #[method(valueForDimension:)]
        pub unsafe fn valueForDimension(&self, dimension: NSTextBlockDimension) -> CGFloat;

        #[method(valueTypeForDimension:)]
        pub unsafe fn valueTypeForDimension(
            &self,
            dimension: NSTextBlockDimension,
        ) -> NSTextBlockValueType;

        #[method(setContentWidth:type:)]
        pub unsafe fn setContentWidth_type(&self, val: CGFloat, r#type: NSTextBlockValueType);

        #[method(contentWidth)]
        pub unsafe fn contentWidth(&self) -> CGFloat;

        #[method(contentWidthValueType)]
        pub unsafe fn contentWidthValueType(&self) -> NSTextBlockValueType;

        #[method(setWidth:type:forLayer:edge:)]
        pub unsafe fn setWidth_type_forLayer_edge(
            &self,
            val: CGFloat,
            r#type: NSTextBlockValueType,
            layer: NSTextBlockLayer,
            edge: NSRectEdge,
        );

        #[method(setWidth:type:forLayer:)]
        pub unsafe fn setWidth_type_forLayer(
            &self,
            val: CGFloat,
            r#type: NSTextBlockValueType,
            layer: NSTextBlockLayer,
        );

        #[method(widthForLayer:edge:)]
        pub unsafe fn widthForLayer_edge(
            &self,
            layer: NSTextBlockLayer,
            edge: NSRectEdge,
        ) -> CGFloat;

        #[method(widthValueTypeForLayer:edge:)]
        pub unsafe fn widthValueTypeForLayer_edge(
            &self,
            layer: NSTextBlockLayer,
            edge: NSRectEdge,
        ) -> NSTextBlockValueType;

        #[method(verticalAlignment)]
        pub unsafe fn verticalAlignment(&self) -> NSTextBlockVerticalAlignment;

        #[method(setVerticalAlignment:)]
        pub unsafe fn setVerticalAlignment(&self, vertical_alignment: NSTextBlockVerticalAlignment);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBorderColor:forEdge:)]
        pub unsafe fn setBorderColor_forEdge(&self, color: Option<&NSColor>, edge: NSRectEdge);

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBorderColor:)]
        pub unsafe fn setBorderColor(&self, color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other borderColorForEdge:)]
        pub unsafe fn borderColorForEdge(&self, edge: NSRectEdge) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSTextContainer")]
        #[method(rectForLayoutAtPoint:inRect:textContainer:characterRange:)]
        pub unsafe fn rectForLayoutAtPoint_inRect_textContainer_characterRange(
            &self,
            starting_point: NSPoint,
            rect: NSRect,
            text_container: &NSTextContainer,
            char_range: NSRange,
        ) -> NSRect;

        #[cfg(feature = "AppKit_NSTextContainer")]
        #[method(boundsRectForContentRect:inRect:textContainer:characterRange:)]
        pub unsafe fn boundsRectForContentRect_inRect_textContainer_characterRange(
            &self,
            content_rect: NSRect,
            rect: NSRect,
            text_container: &NSTextContainer,
            char_range: NSRange,
        ) -> NSRect;

        #[cfg(all(feature = "AppKit_NSLayoutManager", feature = "AppKit_NSView"))]
        #[method(drawBackgroundWithFrame:inView:characterRange:layoutManager:)]
        pub unsafe fn drawBackgroundWithFrame_inView_characterRange_layoutManager(
            &self,
            frame_rect: NSRect,
            control_view: &NSView,
            char_range: NSRange,
            layout_manager: &NSLayoutManager,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTextBlock")]
    unsafe impl NSTextBlock {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextTableBlock")]
    pub struct NSTextTableBlock;

    #[cfg(feature = "AppKit_NSTextTableBlock")]
    unsafe impl ClassType for NSTextTableBlock {
        #[inherits(NSObject)]
        type Super = NSTextBlock;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTextTableBlock")]
unsafe impl NSCoding for NSTextTableBlock {}

#[cfg(feature = "AppKit_NSTextTableBlock")]
unsafe impl NSCopying for NSTextTableBlock {}

#[cfg(feature = "AppKit_NSTextTableBlock")]
unsafe impl NSObjectProtocol for NSTextTableBlock {}

#[cfg(feature = "AppKit_NSTextTableBlock")]
unsafe impl NSSecureCoding for NSTextTableBlock {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextTableBlock")]
    unsafe impl NSTextTableBlock {
        #[cfg(feature = "AppKit_NSTextTable")]
        #[method_id(@__retain_semantics Init initWithTable:startingRow:rowSpan:startingColumn:columnSpan:)]
        pub unsafe fn initWithTable_startingRow_rowSpan_startingColumn_columnSpan(
            this: Option<Allocated<Self>>,
            table: &NSTextTable,
            row: NSInteger,
            row_span: NSInteger,
            col: NSInteger,
            col_span: NSInteger,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSTextTable")]
        #[method_id(@__retain_semantics Other table)]
        pub unsafe fn table(&self) -> Id<NSTextTable>;

        #[method(startingRow)]
        pub unsafe fn startingRow(&self) -> NSInteger;

        #[method(rowSpan)]
        pub unsafe fn rowSpan(&self) -> NSInteger;

        #[method(startingColumn)]
        pub unsafe fn startingColumn(&self) -> NSInteger;

        #[method(columnSpan)]
        pub unsafe fn columnSpan(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextBlock`
    #[cfg(feature = "AppKit_NSTextTableBlock")]
    unsafe impl NSTextTableBlock {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTextTableBlock")]
    unsafe impl NSTextTableBlock {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextTable")]
    pub struct NSTextTable;

    #[cfg(feature = "AppKit_NSTextTable")]
    unsafe impl ClassType for NSTextTable {
        #[inherits(NSObject)]
        type Super = NSTextBlock;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTextTable")]
unsafe impl NSCoding for NSTextTable {}

#[cfg(feature = "AppKit_NSTextTable")]
unsafe impl NSCopying for NSTextTable {}

#[cfg(feature = "AppKit_NSTextTable")]
unsafe impl NSObjectProtocol for NSTextTable {}

#[cfg(feature = "AppKit_NSTextTable")]
unsafe impl NSSecureCoding for NSTextTable {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextTable")]
    unsafe impl NSTextTable {
        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSUInteger;

        #[method(setNumberOfColumns:)]
        pub unsafe fn setNumberOfColumns(&self, number_of_columns: NSUInteger);

        #[method(layoutAlgorithm)]
        pub unsafe fn layoutAlgorithm(&self) -> NSTextTableLayoutAlgorithm;

        #[method(setLayoutAlgorithm:)]
        pub unsafe fn setLayoutAlgorithm(&self, layout_algorithm: NSTextTableLayoutAlgorithm);

        #[method(collapsesBorders)]
        pub unsafe fn collapsesBorders(&self) -> bool;

        #[method(setCollapsesBorders:)]
        pub unsafe fn setCollapsesBorders(&self, collapses_borders: bool);

        #[method(hidesEmptyCells)]
        pub unsafe fn hidesEmptyCells(&self) -> bool;

        #[method(setHidesEmptyCells:)]
        pub unsafe fn setHidesEmptyCells(&self, hides_empty_cells: bool);

        #[cfg(all(
            feature = "AppKit_NSTextContainer",
            feature = "AppKit_NSTextTableBlock"
        ))]
        #[method(rectForBlock:layoutAtPoint:inRect:textContainer:characterRange:)]
        pub unsafe fn rectForBlock_layoutAtPoint_inRect_textContainer_characterRange(
            &self,
            block: &NSTextTableBlock,
            starting_point: NSPoint,
            rect: NSRect,
            text_container: &NSTextContainer,
            char_range: NSRange,
        ) -> NSRect;

        #[cfg(all(
            feature = "AppKit_NSTextContainer",
            feature = "AppKit_NSTextTableBlock"
        ))]
        #[method(boundsRectForBlock:contentRect:inRect:textContainer:characterRange:)]
        pub unsafe fn boundsRectForBlock_contentRect_inRect_textContainer_characterRange(
            &self,
            block: &NSTextTableBlock,
            content_rect: NSRect,
            rect: NSRect,
            text_container: &NSTextContainer,
            char_range: NSRange,
        ) -> NSRect;

        #[cfg(all(
            feature = "AppKit_NSLayoutManager",
            feature = "AppKit_NSTextTableBlock",
            feature = "AppKit_NSView"
        ))]
        #[method(drawBackgroundForBlock:withFrame:inView:characterRange:layoutManager:)]
        pub unsafe fn drawBackgroundForBlock_withFrame_inView_characterRange_layoutManager(
            &self,
            block: &NSTextTableBlock,
            frame_rect: NSRect,
            control_view: &NSView,
            char_range: NSRange,
            layout_manager: &NSLayoutManager,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextBlock`
    #[cfg(feature = "AppKit_NSTextTable")]
    unsafe impl NSTextTable {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTextTable")]
    unsafe impl NSTextTable {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);