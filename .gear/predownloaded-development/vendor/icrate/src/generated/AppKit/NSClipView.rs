//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSClipView")]
    pub struct NSClipView;

    #[cfg(feature = "AppKit_NSClipView")]
    unsafe impl ClassType for NSClipView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSClipView")]
unsafe impl NSAccessibility for NSClipView {}

#[cfg(feature = "AppKit_NSClipView")]
unsafe impl NSAccessibilityElementProtocol for NSClipView {}

#[cfg(feature = "AppKit_NSClipView")]
unsafe impl NSAnimatablePropertyContainer for NSClipView {}

#[cfg(feature = "AppKit_NSClipView")]
unsafe impl NSAppearanceCustomization for NSClipView {}

#[cfg(feature = "AppKit_NSClipView")]
unsafe impl NSCoding for NSClipView {}

#[cfg(feature = "AppKit_NSClipView")]
unsafe impl NSDraggingDestination for NSClipView {}

#[cfg(feature = "AppKit_NSClipView")]
unsafe impl NSObjectProtocol for NSClipView {}

#[cfg(feature = "AppKit_NSClipView")]
unsafe impl NSUserInterfaceItemIdentification for NSClipView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSClipView")]
    unsafe impl NSClipView {
        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[method_id(@__retain_semantics Other documentView)]
        pub unsafe fn documentView(&self) -> Option<Id<NSView>>;

        #[method(setDocumentView:)]
        pub unsafe fn setDocumentView(&self, document_view: Option<&NSView>);

        #[method(documentRect)]
        pub unsafe fn documentRect(&self) -> NSRect;

        #[cfg(feature = "AppKit_NSCursor")]
        #[method_id(@__retain_semantics Other documentCursor)]
        pub unsafe fn documentCursor(&self) -> Option<Id<NSCursor>>;

        #[cfg(feature = "AppKit_NSCursor")]
        #[method(setDocumentCursor:)]
        pub unsafe fn setDocumentCursor(&self, document_cursor: Option<&NSCursor>);

        #[method(documentVisibleRect)]
        pub unsafe fn documentVisibleRect(&self) -> NSRect;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(viewFrameChanged:)]
        pub unsafe fn viewFrameChanged(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(viewBoundsChanged:)]
        pub unsafe fn viewBoundsChanged(&self, notification: &NSNotification);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(autoscroll:)]
        pub unsafe fn autoscroll(&self, event: &NSEvent) -> bool;

        #[method(scrollToPoint:)]
        pub unsafe fn scrollToPoint(&self, new_origin: NSPoint);

        #[method(constrainBoundsRect:)]
        pub unsafe fn constrainBoundsRect(&self, proposed_bounds: NSRect) -> NSRect;

        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSEdgeInsets;

        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, content_insets: NSEdgeInsets);

        #[method(automaticallyAdjustsContentInsets)]
        pub unsafe fn automaticallyAdjustsContentInsets(&self) -> bool;

        #[method(setAutomaticallyAdjustsContentInsets:)]
        pub unsafe fn setAutomaticallyAdjustsContentInsets(
            &self,
            automatically_adjusts_content_insets: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSClipView")]
    unsafe impl NSClipView {
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
    #[cfg(feature = "AppKit_NSClipView")]
    unsafe impl NSClipView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSClipView")]
    unsafe impl NSClipView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSClipViewSuperview
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[cfg(feature = "AppKit_NSClipView")]
        #[method(reflectScrolledClipView:)]
        pub unsafe fn reflectScrolledClipView(&self, clip_view: &NSClipView);

        #[cfg(feature = "AppKit_NSClipView")]
        #[method(scrollClipView:toPoint:)]
        pub unsafe fn scrollClipView_toPoint(&self, clip_view: &NSClipView, point: NSPoint);
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSClipView")]
    unsafe impl NSClipView {
        #[deprecated = "Use -constrainBoundsRect: instead."]
        #[method(constrainScrollPoint:)]
        pub unsafe fn constrainScrollPoint(&self, new_origin: NSPoint) -> NSPoint;

        #[deprecated = "Setting this property has no effect.  NSClipView will always minimize the area of the document view that is invalidated.  To force invalidation of the document view, use -[NSView setNeedsDisplayInRect:]."]
        #[method(copiesOnScroll)]
        pub unsafe fn copiesOnScroll(&self) -> bool;

        #[deprecated = "Setting this property has no effect.  NSClipView will always minimize the area of the document view that is invalidated.  To force invalidation of the document view, use -[NSView setNeedsDisplayInRect:]."]
        #[method(setCopiesOnScroll:)]
        pub unsafe fn setCopiesOnScroll(&self, copies_on_scroll: bool);
    }
);