//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSWindowTabGroup")]
    pub struct NSWindowTabGroup;

    #[cfg(feature = "AppKit_NSWindowTabGroup")]
    unsafe impl ClassType for NSWindowTabGroup {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSWindowTabGroup")]
unsafe impl NSObjectProtocol for NSWindowTabGroup {}

extern_methods!(
    #[cfg(feature = "AppKit_NSWindowTabGroup")]
    unsafe impl NSWindowTabGroup {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSWindowTabbingIdentifier>;

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other windows)]
        pub unsafe fn windows(&self) -> Id<NSArray<NSWindow>>;

        #[method(isOverviewVisible)]
        pub unsafe fn isOverviewVisible(&self) -> bool;

        #[method(setOverviewVisible:)]
        pub unsafe fn setOverviewVisible(&self, overview_visible: bool);

        #[method(isTabBarVisible)]
        pub unsafe fn isTabBarVisible(&self) -> bool;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other selectedWindow)]
        pub unsafe fn selectedWindow(&self) -> Option<Id<NSWindow>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setSelectedWindow:)]
        pub unsafe fn setSelectedWindow(&self, selected_window: Option<&NSWindow>);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(addWindow:)]
        pub unsafe fn addWindow(&self, window: &NSWindow);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(insertWindow:atIndex:)]
        pub unsafe fn insertWindow_atIndex(&self, window: &NSWindow, index: NSInteger);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(removeWindow:)]
        pub unsafe fn removeWindow(&self, window: &NSWindow);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSWindowTabGroup")]
    unsafe impl NSWindowTabGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
