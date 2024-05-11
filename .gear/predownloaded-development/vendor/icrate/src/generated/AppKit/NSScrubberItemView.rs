//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    pub struct NSScrubberArrangedView;

    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    unsafe impl ClassType for NSScrubberArrangedView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSAccessibility for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSAppearanceCustomization for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSCoding for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSDraggingDestination for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSObjectProtocol for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberArrangedView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    unsafe impl NSScrubberArrangedView {
        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[cfg(feature = "AppKit_NSScrubberLayoutAttributes")]
        #[method(applyLayoutAttributes:)]
        pub unsafe fn applyLayoutAttributes(&self, layout_attributes: &NSScrubberLayoutAttributes);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    unsafe impl NSScrubberArrangedView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    unsafe impl NSScrubberArrangedView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    unsafe impl NSScrubberArrangedView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    pub struct NSScrubberSelectionView;

    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    unsafe impl ClassType for NSScrubberSelectionView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSScrubberArrangedView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSAccessibility for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSAppearanceCustomization for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSCoding for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSDraggingDestination for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSObjectProtocol for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberSelectionView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    unsafe impl NSScrubberSelectionView {}
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    unsafe impl NSScrubberSelectionView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    unsafe impl NSScrubberSelectionView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    unsafe impl NSScrubberSelectionView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberItemView")]
    pub struct NSScrubberItemView;

    #[cfg(feature = "AppKit_NSScrubberItemView")]
    unsafe impl ClassType for NSScrubberItemView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSScrubberArrangedView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSAccessibility for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSAppearanceCustomization for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSCoding for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSDraggingDestination for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSObjectProtocol for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberItemView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberItemView")]
    unsafe impl NSScrubberItemView {}
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberItemView")]
    unsafe impl NSScrubberItemView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSScrubberItemView")]
    unsafe impl NSScrubberItemView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSScrubberItemView")]
    unsafe impl NSScrubberItemView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    pub struct NSScrubberTextItemView;

    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    unsafe impl ClassType for NSScrubberTextItemView {
        #[inherits(NSScrubberArrangedView, NSView, NSResponder, NSObject)]
        type Super = NSScrubberItemView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSAccessibility for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSAppearanceCustomization for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSCoding for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSDraggingDestination for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSObjectProtocol for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberTextItemView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    unsafe impl NSScrubberTextItemView {
        #[cfg(feature = "AppKit_NSTextField")]
        #[method_id(@__retain_semantics Other textField)]
        pub unsafe fn textField(&self) -> Id<NSTextField>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    unsafe impl NSScrubberTextItemView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    unsafe impl NSScrubberTextItemView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    unsafe impl NSScrubberTextItemView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    pub struct NSScrubberImageItemView;

    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    unsafe impl ClassType for NSScrubberImageItemView {
        #[inherits(NSScrubberArrangedView, NSView, NSResponder, NSObject)]
        type Super = NSScrubberItemView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSAccessibility for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSAppearanceCustomization for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSCoding for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSDraggingDestination for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSObjectProtocol for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberImageItemView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    unsafe impl NSScrubberImageItemView {
        #[cfg(feature = "AppKit_NSImageView")]
        #[method_id(@__retain_semantics Other imageView)]
        pub unsafe fn imageView(&self) -> Id<NSImageView>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<NSImage>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: &NSImage);

        #[method(imageAlignment)]
        pub unsafe fn imageAlignment(&self) -> NSImageAlignment;

        #[method(setImageAlignment:)]
        pub unsafe fn setImageAlignment(&self, image_alignment: NSImageAlignment);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    unsafe impl NSScrubberImageItemView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    unsafe impl NSScrubberImageItemView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    unsafe impl NSScrubberImageItemView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);