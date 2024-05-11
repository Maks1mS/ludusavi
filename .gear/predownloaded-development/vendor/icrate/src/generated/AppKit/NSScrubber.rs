//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSScrubberDataSource: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSScrubber")]
        #[method(numberOfItemsForScrubber:)]
        unsafe fn numberOfItemsForScrubber(&self, scrubber: &NSScrubber) -> NSInteger;

        #[cfg(all(feature = "AppKit_NSScrubber", feature = "AppKit_NSScrubberItemView"))]
        #[method_id(@__retain_semantics Other scrubber:viewForItemAtIndex:)]
        unsafe fn scrubber_viewForItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            index: NSInteger,
        ) -> Id<NSScrubberItemView>;
    }

    unsafe impl ProtocolType for dyn NSScrubberDataSource {}
);

extern_protocol!(
    pub unsafe trait NSScrubberDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSScrubber")]
        #[optional]
        #[method(scrubber:didSelectItemAtIndex:)]
        unsafe fn scrubber_didSelectItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            selected_index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSScrubber")]
        #[optional]
        #[method(scrubber:didHighlightItemAtIndex:)]
        unsafe fn scrubber_didHighlightItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            highlighted_index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSScrubber")]
        #[optional]
        #[method(scrubber:didChangeVisibleRange:)]
        unsafe fn scrubber_didChangeVisibleRange(
            &self,
            scrubber: &NSScrubber,
            visible_range: NSRange,
        );

        #[cfg(feature = "AppKit_NSScrubber")]
        #[optional]
        #[method(didBeginInteractingWithScrubber:)]
        unsafe fn didBeginInteractingWithScrubber(&self, scrubber: &NSScrubber);

        #[cfg(feature = "AppKit_NSScrubber")]
        #[optional]
        #[method(didFinishInteractingWithScrubber:)]
        unsafe fn didFinishInteractingWithScrubber(&self, scrubber: &NSScrubber);

        #[cfg(feature = "AppKit_NSScrubber")]
        #[optional]
        #[method(didCancelInteractingWithScrubber:)]
        unsafe fn didCancelInteractingWithScrubber(&self, scrubber: &NSScrubber);
    }

    unsafe impl ProtocolType for dyn NSScrubberDelegate {}
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSScrubberMode {
        NSScrubberModeFixed = 0,
        NSScrubberModeFree = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSScrubberAlignment {
        NSScrubberAlignmentNone = 0,
        NSScrubberAlignmentLeading = 1,
        NSScrubberAlignmentTrailing = 2,
        NSScrubberAlignmentCenter = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
    pub struct NSScrubberSelectionStyle;

    #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
    unsafe impl ClassType for NSScrubberSelectionStyle {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
unsafe impl NSCoding for NSScrubberSelectionStyle {}

#[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
unsafe impl NSObjectProtocol for NSScrubberSelectionStyle {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
    unsafe impl NSScrubberSelectionStyle {
        #[method_id(@__retain_semantics Other outlineOverlayStyle)]
        pub unsafe fn outlineOverlayStyle() -> Id<NSScrubberSelectionStyle>;

        #[method_id(@__retain_semantics Other roundedBackgroundStyle)]
        pub unsafe fn roundedBackgroundStyle() -> Id<NSScrubberSelectionStyle>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSScrubberSelectionView")]
        #[method_id(@__retain_semantics Other makeSelectionView)]
        pub unsafe fn makeSelectionView(&self) -> Option<Id<NSScrubberSelectionView>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
    unsafe impl NSScrubberSelectionStyle {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubber")]
    pub struct NSScrubber;

    #[cfg(feature = "AppKit_NSScrubber")]
    unsafe impl ClassType for NSScrubber {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSAccessibility for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSAccessibilityElementProtocol for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSAnimatablePropertyContainer for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSAppearanceCustomization for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSCoding for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSDraggingDestination for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSObjectProtocol for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubber {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubber")]
    unsafe impl NSScrubber {
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<ProtocolObject<dyn NSScrubberDataSource>>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn NSScrubberDataSource>>,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSScrubberDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSScrubberDelegate>>);

        #[cfg(feature = "AppKit_NSScrubberLayout")]
        #[method_id(@__retain_semantics Other scrubberLayout)]
        pub unsafe fn scrubberLayout(&self) -> Id<NSScrubberLayout>;

        #[cfg(feature = "AppKit_NSScrubberLayout")]
        #[method(setScrubberLayout:)]
        pub unsafe fn setScrubberLayout(&self, scrubber_layout: &NSScrubberLayout);

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(highlightedIndex)]
        pub unsafe fn highlightedIndex(&self) -> NSInteger;

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSInteger);

        #[method(mode)]
        pub unsafe fn mode(&self) -> NSScrubberMode;

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSScrubberMode);

        #[method(itemAlignment)]
        pub unsafe fn itemAlignment(&self) -> NSScrubberAlignment;

        #[method(setItemAlignment:)]
        pub unsafe fn setItemAlignment(&self, item_alignment: NSScrubberAlignment);

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(floatsSelectionViews)]
        pub unsafe fn floatsSelectionViews(&self) -> bool;

        #[method(setFloatsSelectionViews:)]
        pub unsafe fn setFloatsSelectionViews(&self, floats_selection_views: bool);

        #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
        #[method_id(@__retain_semantics Other selectionBackgroundStyle)]
        pub unsafe fn selectionBackgroundStyle(&self) -> Option<Id<NSScrubberSelectionStyle>>;

        #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
        #[method(setSelectionBackgroundStyle:)]
        pub unsafe fn setSelectionBackgroundStyle(
            &self,
            selection_background_style: Option<&NSScrubberSelectionStyle>,
        );

        #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
        #[method_id(@__retain_semantics Other selectionOverlayStyle)]
        pub unsafe fn selectionOverlayStyle(&self) -> Option<Id<NSScrubberSelectionStyle>>;

        #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
        #[method(setSelectionOverlayStyle:)]
        pub unsafe fn setSelectionOverlayStyle(
            &self,
            selection_overlay_style: Option<&NSScrubberSelectionStyle>,
        );

        #[method(showsArrowButtons)]
        pub unsafe fn showsArrowButtons(&self) -> bool;

        #[method(setShowsArrowButtons:)]
        pub unsafe fn setShowsArrowButtons(&self, shows_arrow_buttons: bool);

        #[method(showsAdditionalContentIndicators)]
        pub unsafe fn showsAdditionalContentIndicators(&self) -> bool;

        #[method(setShowsAdditionalContentIndicators:)]
        pub unsafe fn setShowsAdditionalContentIndicators(
            &self,
            shows_additional_content_indicators: bool,
        );

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[method_id(@__retain_semantics Other backgroundView)]
        pub unsafe fn backgroundView(&self) -> Option<Id<NSView>>;

        #[method(setBackgroundView:)]
        pub unsafe fn setBackgroundView(&self, background_view: Option<&NSView>);

        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[method(performSequentialBatchUpdates:)]
        pub unsafe fn performSequentialBatchUpdates(&self, update_block: &Block<(), ()>);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(insertItemsAtIndexes:)]
        pub unsafe fn insertItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeItemsAtIndexes:)]
        pub unsafe fn removeItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(reloadItemsAtIndexes:)]
        pub unsafe fn reloadItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[method(moveItemAtIndex:toIndex:)]
        pub unsafe fn moveItemAtIndex_toIndex(&self, old_index: NSInteger, new_index: NSInteger);

        #[method(scrollItemAtIndex:toAlignment:)]
        pub unsafe fn scrollItemAtIndex_toAlignment(
            &self,
            index: NSInteger,
            alignment: NSScrubberAlignment,
        );

        #[cfg(feature = "AppKit_NSScrubberItemView")]
        #[method_id(@__retain_semantics Other itemViewForItemAtIndex:)]
        pub unsafe fn itemViewForItemAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<NSScrubberItemView>>;

        #[method(registerClass:forItemIdentifier:)]
        pub unsafe fn registerClass_forItemIdentifier(
            &self,
            item_view_class: Option<&AnyClass>,
            item_identifier: &NSUserInterfaceItemIdentifier,
        );

        #[cfg(feature = "AppKit_NSNib")]
        #[method(registerNib:forItemIdentifier:)]
        pub unsafe fn registerNib_forItemIdentifier(
            &self,
            nib: Option<&NSNib>,
            item_identifier: &NSUserInterfaceItemIdentifier,
        );

        #[cfg(feature = "AppKit_NSScrubberItemView")]
        #[method_id(@__retain_semantics Other makeItemWithIdentifier:owner:)]
        pub unsafe fn makeItemWithIdentifier_owner(
            &self,
            item_identifier: &NSUserInterfaceItemIdentifier,
            owner: Option<&AnyObject>,
        ) -> Option<Id<NSScrubberItemView>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSScrubber")]
    unsafe impl NSScrubber {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSScrubber")]
    unsafe impl NSScrubber {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);