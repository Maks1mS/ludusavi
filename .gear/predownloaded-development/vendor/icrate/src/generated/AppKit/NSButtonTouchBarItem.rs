//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSButtonTouchBarItem")]
    pub struct NSButtonTouchBarItem;

    #[cfg(feature = "AppKit_NSButtonTouchBarItem")]
    unsafe impl ClassType for NSButtonTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSButtonTouchBarItem")]
unsafe impl NSCoding for NSButtonTouchBarItem {}

#[cfg(feature = "AppKit_NSButtonTouchBarItem")]
unsafe impl NSObjectProtocol for NSButtonTouchBarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSButtonTouchBarItem")]
    unsafe impl NSButtonTouchBarItem {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other buttonTouchBarItemWithIdentifier:title:target:action:)]
        pub unsafe fn buttonTouchBarItemWithIdentifier_title_target_action(
            identifier: &NSTouchBarItemIdentifier,
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other buttonTouchBarItemWithIdentifier:image:target:action:)]
        pub unsafe fn buttonTouchBarItemWithIdentifier_image_target_action(
            identifier: &NSTouchBarItemIdentifier,
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other buttonTouchBarItemWithIdentifier:title:image:target:action:)]
        pub unsafe fn buttonTouchBarItemWithIdentifier_title_image_target_action(
            identifier: &NSTouchBarItemIdentifier,
            title: &NSString,
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other bezelColor)]
        pub unsafe fn bezelColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBezelColor:)]
        pub unsafe fn setBezelColor(&self, bezel_color: Option<&NSColor>);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<AnyObject>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSButtonTouchBarItem")]
    unsafe impl NSButtonTouchBarItem {
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
    #[cfg(feature = "AppKit_NSButtonTouchBarItem")]
    unsafe impl NSButtonTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);