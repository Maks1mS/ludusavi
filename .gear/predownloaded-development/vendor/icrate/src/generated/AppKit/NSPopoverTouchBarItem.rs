//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPopoverTouchBarItem")]
    pub struct NSPopoverTouchBarItem;

    #[cfg(feature = "AppKit_NSPopoverTouchBarItem")]
    unsafe impl ClassType for NSPopoverTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSPopoverTouchBarItem")]
unsafe impl NSCoding for NSPopoverTouchBarItem {}

#[cfg(feature = "AppKit_NSPopoverTouchBarItem")]
unsafe impl NSObjectProtocol for NSPopoverTouchBarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPopoverTouchBarItem")]
    unsafe impl NSPopoverTouchBarItem {
        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method_id(@__retain_semantics Other popoverTouchBar)]
        pub unsafe fn popoverTouchBar(&self) -> Id<NSTouchBar>;

        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method(setPopoverTouchBar:)]
        pub unsafe fn setPopoverTouchBar(&self, popover_touch_bar: &NSTouchBar);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other collapsedRepresentation)]
        pub unsafe fn collapsedRepresentation(&self) -> Id<NSView>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setCollapsedRepresentation:)]
        pub unsafe fn setCollapsedRepresentation(&self, collapsed_representation: &NSView);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other collapsedRepresentationImage)]
        pub unsafe fn collapsedRepresentationImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setCollapsedRepresentationImage:)]
        pub unsafe fn setCollapsedRepresentationImage(
            &self,
            collapsed_representation_image: Option<&NSImage>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other collapsedRepresentationLabel)]
        pub unsafe fn collapsedRepresentationLabel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCollapsedRepresentationLabel:)]
        pub unsafe fn setCollapsedRepresentationLabel(
            &self,
            collapsed_representation_label: &NSString,
        );

        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method_id(@__retain_semantics Other pressAndHoldTouchBar)]
        pub unsafe fn pressAndHoldTouchBar(&self) -> Option<Id<NSTouchBar>>;

        #[cfg(feature = "AppKit_NSTouchBar")]
        #[method(setPressAndHoldTouchBar:)]
        pub unsafe fn setPressAndHoldTouchBar(&self, press_and_hold_touch_bar: Option<&NSTouchBar>);

        #[method(showsCloseButton)]
        pub unsafe fn showsCloseButton(&self) -> bool;

        #[method(setShowsCloseButton:)]
        pub unsafe fn setShowsCloseButton(&self, shows_close_button: bool);

        #[method(showPopover:)]
        pub unsafe fn showPopover(&self, sender: Option<&AnyObject>);

        #[method(dismissPopover:)]
        pub unsafe fn dismissPopover(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "AppKit_NSGestureRecognizer")]
        #[method_id(@__retain_semantics Other makeStandardActivatePopoverGestureRecognizer)]
        pub unsafe fn makeStandardActivatePopoverGestureRecognizer(
            &self,
        ) -> Id<NSGestureRecognizer>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSPopoverTouchBarItem")]
    unsafe impl NSPopoverTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSPopoverTouchBarItem")]
    unsafe impl NSPopoverTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);