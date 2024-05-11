//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSOpenPanel")]
    pub struct NSOpenPanel;

    #[cfg(feature = "AppKit_NSOpenPanel")]
    unsafe impl ClassType for NSOpenPanel {
        #[inherits(NSPanel, NSWindow, NSResponder, NSObject)]
        type Super = NSSavePanel;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSOpenPanel")]
unsafe impl NSAccessibility for NSOpenPanel {}

#[cfg(feature = "AppKit_NSOpenPanel")]
unsafe impl NSAccessibilityElementProtocol for NSOpenPanel {}

#[cfg(feature = "AppKit_NSOpenPanel")]
unsafe impl NSAnimatablePropertyContainer for NSOpenPanel {}

#[cfg(feature = "AppKit_NSOpenPanel")]
unsafe impl NSAppearanceCustomization for NSOpenPanel {}

#[cfg(feature = "AppKit_NSOpenPanel")]
unsafe impl NSCoding for NSOpenPanel {}

#[cfg(feature = "AppKit_NSOpenPanel")]
unsafe impl NSMenuItemValidation for NSOpenPanel {}

#[cfg(feature = "AppKit_NSOpenPanel")]
unsafe impl NSObjectProtocol for NSOpenPanel {}

#[cfg(feature = "AppKit_NSOpenPanel")]
unsafe impl NSUserInterfaceItemIdentification for NSOpenPanel {}

#[cfg(feature = "AppKit_NSOpenPanel")]
unsafe impl NSUserInterfaceValidations for NSOpenPanel {}

extern_methods!(
    #[cfg(feature = "AppKit_NSOpenPanel")]
    unsafe impl NSOpenPanel {
        #[method_id(@__retain_semantics Other openPanel)]
        pub unsafe fn openPanel() -> Id<NSOpenPanel>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLs)]
        pub unsafe fn URLs(&self) -> Id<NSArray<NSURL>>;

        #[method(resolvesAliases)]
        pub unsafe fn resolvesAliases(&self) -> bool;

        #[method(setResolvesAliases:)]
        pub unsafe fn setResolvesAliases(&self, resolves_aliases: bool);

        #[method(canChooseDirectories)]
        pub unsafe fn canChooseDirectories(&self) -> bool;

        #[method(setCanChooseDirectories:)]
        pub unsafe fn setCanChooseDirectories(&self, can_choose_directories: bool);

        #[method(allowsMultipleSelection)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;

        #[method(setAllowsMultipleSelection:)]
        pub unsafe fn setAllowsMultipleSelection(&self, allows_multiple_selection: bool);

        #[method(canChooseFiles)]
        pub unsafe fn canChooseFiles(&self) -> bool;

        #[method(setCanChooseFiles:)]
        pub unsafe fn setCanChooseFiles(&self, can_choose_files: bool);

        #[method(canResolveUbiquitousConflicts)]
        pub unsafe fn canResolveUbiquitousConflicts(&self) -> bool;

        #[method(setCanResolveUbiquitousConflicts:)]
        pub unsafe fn setCanResolveUbiquitousConflicts(
            &self,
            can_resolve_ubiquitous_conflicts: bool,
        );

        #[method(canDownloadUbiquitousContents)]
        pub unsafe fn canDownloadUbiquitousContents(&self) -> bool;

        #[method(setCanDownloadUbiquitousContents:)]
        pub unsafe fn setCanDownloadUbiquitousContents(
            &self,
            can_download_ubiquitous_contents: bool,
        );

        #[method(isAccessoryViewDisclosed)]
        pub unsafe fn isAccessoryViewDisclosed(&self) -> bool;

        #[method(setAccessoryViewDisclosed:)]
        pub unsafe fn setAccessoryViewDisclosed(&self, accessory_view_disclosed: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(feature = "AppKit_NSOpenPanel")]
    unsafe impl NSOpenPanel {
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Option<Allocated<Self>>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSScreen")]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Option<Allocated<Self>>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            content_view_controller: &NSViewController,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSOpenPanel")]
    unsafe impl NSOpenPanel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSOpenPanel")]
    unsafe impl NSOpenPanel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSOpenPanel")]
    unsafe impl NSOpenPanel {
        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method_id(@__retain_semantics Other filenames)]
        pub unsafe fn filenames(&self) -> Id<NSArray>;

        #[cfg(all(
            feature = "AppKit_NSWindow",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[deprecated]
        #[method(beginSheetForDirectory:file:types:modalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetForDirectory_file_types_modalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
            file_types: Option<&NSArray>,
            doc_window: Option<&NSWindow>,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(beginForDirectory:file:types:modelessDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginForDirectory_file_types_modelessDelegate_didEndSelector_contextInfo(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
            file_types: Option<&NSArray>,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(runModalForDirectory:file:types:)]
        pub unsafe fn runModalForDirectory_file_types(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
            file_types: Option<&NSArray>,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method(runModalForTypes:)]
        pub unsafe fn runModalForTypes(&self, file_types: Option<&NSArray>) -> NSInteger;
    }
);