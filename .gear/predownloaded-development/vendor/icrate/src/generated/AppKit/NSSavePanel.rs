//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSavePanel")]
    pub struct NSSavePanel;

    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl ClassType for NSSavePanel {
        #[inherits(NSWindow, NSResponder, NSObject)]
        type Super = NSPanel;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSSavePanel")]
unsafe impl NSAccessibility for NSSavePanel {}

#[cfg(feature = "AppKit_NSSavePanel")]
unsafe impl NSAccessibilityElementProtocol for NSSavePanel {}

#[cfg(feature = "AppKit_NSSavePanel")]
unsafe impl NSAnimatablePropertyContainer for NSSavePanel {}

#[cfg(feature = "AppKit_NSSavePanel")]
unsafe impl NSAppearanceCustomization for NSSavePanel {}

#[cfg(feature = "AppKit_NSSavePanel")]
unsafe impl NSCoding for NSSavePanel {}

#[cfg(feature = "AppKit_NSSavePanel")]
unsafe impl NSMenuItemValidation for NSSavePanel {}

#[cfg(feature = "AppKit_NSSavePanel")]
unsafe impl NSObjectProtocol for NSSavePanel {}

#[cfg(feature = "AppKit_NSSavePanel")]
unsafe impl NSUserInterfaceItemIdentification for NSSavePanel {}

#[cfg(feature = "AppKit_NSSavePanel")]
unsafe impl NSUserInterfaceValidations for NSSavePanel {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl NSSavePanel {
        #[method_id(@__retain_semantics Other savePanel)]
        pub unsafe fn savePanel() -> Id<NSSavePanel>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other directoryURL)]
        pub unsafe fn directoryURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setDirectoryURL:)]
        pub unsafe fn setDirectoryURL(&self, directory_url: Option<&NSURL>);

        #[method(allowsOtherFileTypes)]
        pub unsafe fn allowsOtherFileTypes(&self) -> bool;

        #[method(setAllowsOtherFileTypes:)]
        pub unsafe fn setAllowsOtherFileTypes(&self, allows_other_file_types: bool);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSOpenSavePanelDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSOpenSavePanelDelegate>>,
        );

        #[method(isExpanded)]
        pub unsafe fn isExpanded(&self) -> bool;

        #[method(canCreateDirectories)]
        pub unsafe fn canCreateDirectories(&self) -> bool;

        #[method(setCanCreateDirectories:)]
        pub unsafe fn setCanCreateDirectories(&self, can_create_directories: bool);

        #[method(canSelectHiddenExtension)]
        pub unsafe fn canSelectHiddenExtension(&self) -> bool;

        #[method(setCanSelectHiddenExtension:)]
        pub unsafe fn setCanSelectHiddenExtension(&self, can_select_hidden_extension: bool);

        #[method(isExtensionHidden)]
        pub unsafe fn isExtensionHidden(&self) -> bool;

        #[method(setExtensionHidden:)]
        pub unsafe fn setExtensionHidden(&self, extension_hidden: bool);

        #[method(treatsFilePackagesAsDirectories)]
        pub unsafe fn treatsFilePackagesAsDirectories(&self) -> bool;

        #[method(setTreatsFilePackagesAsDirectories:)]
        pub unsafe fn setTreatsFilePackagesAsDirectories(
            &self,
            treats_file_packages_as_directories: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other prompt)]
        pub unsafe fn prompt(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPrompt:)]
        pub unsafe fn setPrompt(&self, prompt: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nameFieldLabel)]
        pub unsafe fn nameFieldLabel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNameFieldLabel:)]
        pub unsafe fn setNameFieldLabel(&self, name_field_label: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nameFieldStringValue)]
        pub unsafe fn nameFieldStringValue(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNameFieldStringValue:)]
        pub unsafe fn setNameFieldStringValue(&self, name_field_string_value: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other message)]
        pub unsafe fn message(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMessage:)]
        pub unsafe fn setMessage(&self, message: Option<&NSString>);

        #[method(validateVisibleColumns)]
        pub unsafe fn validateVisibleColumns(&self);

        #[method(showsHiddenFiles)]
        pub unsafe fn showsHiddenFiles(&self) -> bool;

        #[method(setShowsHiddenFiles:)]
        pub unsafe fn setShowsHiddenFiles(&self, shows_hidden_files: bool);

        #[method(showsTagField)]
        pub unsafe fn showsTagField(&self) -> bool;

        #[method(setShowsTagField:)]
        pub unsafe fn setShowsTagField(&self, shows_tag_field: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other tagNames)]
        pub unsafe fn tagNames(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setTagNames:)]
        pub unsafe fn setTagNames(&self, tag_names: Option<&NSArray<NSString>>);

        #[method(ok:)]
        pub unsafe fn ok(&self, sender: Option<&AnyObject>);

        #[method(cancel:)]
        pub unsafe fn cancel(&self, sender: Option<&AnyObject>);

        #[method(beginSheetModalForWindow:completionHandler:)]
        pub unsafe fn beginSheetModalForWindow_completionHandler(
            &self,
            window: &NSWindow,
            handler: &Block<(NSModalResponse,), ()>,
        );

        #[method(beginWithCompletionHandler:)]
        pub unsafe fn beginWithCompletionHandler(&self, handler: &Block<(NSModalResponse,), ()>);

        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSModalResponse;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl NSSavePanel {
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
    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl NSSavePanel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl NSSavePanel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSOpenSavePanelDelegate: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSURL")]
        #[optional]
        #[method(panel:shouldEnableURL:)]
        unsafe fn panel_shouldEnableURL(&self, sender: &AnyObject, url: &NSURL) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[optional]
        #[method(panel:validateURL:error:_)]
        unsafe fn panel_validateURL_error(
            &self,
            sender: &AnyObject,
            url: &NSURL,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[optional]
        #[method(panel:didChangeToDirectoryURL:)]
        unsafe fn panel_didChangeToDirectoryURL(&self, sender: &AnyObject, url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method_id(@__retain_semantics Other panel:userEnteredFilename:confirmed:)]
        unsafe fn panel_userEnteredFilename_confirmed(
            &self,
            sender: &AnyObject,
            filename: &NSString,
            ok_flag: bool,
        ) -> Option<Id<NSString>>;

        #[optional]
        #[method(panel:willExpand:)]
        unsafe fn panel_willExpand(&self, sender: &AnyObject, expanding: bool);

        #[optional]
        #[method(panelSelectionDidChange:)]
        unsafe fn panelSelectionDidChange(&self, sender: Option<&AnyObject>);
    }

    unsafe impl ProtocolType for dyn NSOpenSavePanelDelegate {}
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl NSSavePanel {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -URL instead"]
        #[method_id(@__retain_semantics Other filename)]
        pub unsafe fn filename(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -directoryURL instead"]
        #[method_id(@__retain_semantics Other directory)]
        pub unsafe fn directory(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -setDirectoryURL: instead"]
        #[method(setDirectory:)]
        pub unsafe fn setDirectory(&self, path: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -allowedFileTypes instead"]
        #[method_id(@__retain_semantics Other requiredFileType)]
        pub unsafe fn requiredFileType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -setAllowedFileTypes: instead"]
        #[method(setRequiredFileType:)]
        pub unsafe fn setRequiredFileType(&self, r#type: Option<&NSString>);

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSString"))]
        #[deprecated = "Use beginSheetModalForWindow:completionHandler: instead. The following parameters are replaced by properties: 'path' is replaced by 'directoryURL' and 'name' by 'nameFieldStringValue'."]
        #[method(beginSheetForDirectory:file:modalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetForDirectory_file_modalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            path: &NSString,
            name: Option<&NSString>,
            doc_window: Option<&NSWindow>,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -runModal instead. The following parameters are replaced by properties: 'path' is replaced by 'directoryURL' and 'name' by 'nameFieldStringValue'."]
        #[method(runModalForDirectory:file:)]
        pub unsafe fn runModalForDirectory_file(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
        ) -> NSInteger;

        #[deprecated = "Default implementation does nothing."]
        #[method(selectText:)]
        pub unsafe fn selectText(&self, sender: Option<&AnyObject>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -allowedContentTypes instead"]
        #[method_id(@__retain_semantics Other allowedFileTypes)]
        pub unsafe fn allowedFileTypes(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -allowedContentTypes instead"]
        #[method(setAllowedFileTypes:)]
        pub unsafe fn setAllowedFileTypes(&self, allowed_file_types: Option<&NSArray<NSString>>);
    }
);
