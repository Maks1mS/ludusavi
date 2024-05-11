//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::InputMethodKit::*;

extern_protocol!(
    pub unsafe trait IMKStateSetting {
        #[method(activateServer:)]
        unsafe fn activateServer(&self, sender: Option<&AnyObject>);

        #[method(deactivateServer:)]
        unsafe fn deactivateServer(&self, sender: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other valueForTag:client:)]
        unsafe fn valueForTag_client(
            &self,
            tag: c_long,
            sender: Option<&AnyObject>,
        ) -> Option<Id<AnyObject>>;

        #[method(setValue:forTag:client:)]
        unsafe fn setValue_forTag_client(
            &self,
            value: Option<&AnyObject>,
            tag: c_long,
            sender: Option<&AnyObject>,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other modes:)]
        unsafe fn modes(&self, sender: Option<&AnyObject>) -> Option<Id<NSDictionary>>;

        #[method(recognizedEvents:)]
        unsafe fn recognizedEvents(&self, sender: Option<&AnyObject>) -> NSUInteger;

        #[method(showPreferences:)]
        unsafe fn showPreferences(&self, sender: Option<&AnyObject>);
    }

    unsafe impl ProtocolType for dyn IMKStateSetting {}
);

extern_protocol!(
    pub unsafe trait IMKMouseHandling {
        #[method(mouseDownOnCharacterIndex:coordinate:withModifier:continueTracking:client:)]
        unsafe fn mouseDownOnCharacterIndex_coordinate_withModifier_continueTracking_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            keep_tracking: *mut Bool,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[method(mouseUpOnCharacterIndex:coordinate:withModifier:client:)]
        unsafe fn mouseUpOnCharacterIndex_coordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[method(mouseMovedOnCharacterIndex:coordinate:withModifier:client:)]
        unsafe fn mouseMovedOnCharacterIndex_coordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&AnyObject>,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn IMKMouseHandling {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "InputMethodKit_IMKInputController")]
    pub struct IMKInputController;

    #[cfg(feature = "InputMethodKit_IMKInputController")]
    unsafe impl ClassType for IMKInputController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "InputMethodKit_IMKInputController")]
unsafe impl IMKMouseHandling for IMKInputController {}

#[cfg(feature = "InputMethodKit_IMKInputController")]
unsafe impl IMKStateSetting for IMKInputController {}

#[cfg(feature = "InputMethodKit_IMKInputController")]
unsafe impl NSObjectProtocol for IMKInputController {}

extern_methods!(
    #[cfg(feature = "InputMethodKit_IMKInputController")]
    unsafe impl IMKInputController {
        #[cfg(feature = "InputMethodKit_IMKServer")]
        #[method_id(@__retain_semantics Init initWithServer:delegate:client:)]
        pub unsafe fn initWithServer_delegate_client(
            this: Option<Allocated<Self>>,
            server: Option<&IMKServer>,
            delegate: Option<&AnyObject>,
            input_client: Option<&AnyObject>,
        ) -> Option<Id<Self>>;

        #[method(updateComposition)]
        pub unsafe fn updateComposition(&self);

        #[method(cancelComposition)]
        pub unsafe fn cancelComposition(&self);

        #[cfg(feature = "Foundation_NSMutableDictionary")]
        #[method_id(@__retain_semantics Other compositionAttributesAtRange:)]
        pub unsafe fn compositionAttributesAtRange(
            &self,
            range: NSRange,
        ) -> Option<Id<NSMutableDictionary>>;

        #[method(selectionRange)]
        pub unsafe fn selectionRange(&self) -> NSRange;

        #[method(replacementRange)]
        pub unsafe fn replacementRange(&self) -> NSRange;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other markForStyle:atRange:)]
        pub unsafe fn markForStyle_atRange(
            &self,
            style: NSInteger,
            range: NSRange,
        ) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(doCommandBySelector:commandDictionary:)]
        pub unsafe fn doCommandBySelector_commandDictionary(
            &self,
            a_selector: Option<Sel>,
            info_dictionary: Option<&NSDictionary>,
        );

        #[method(hidePalettes)]
        pub unsafe fn hidePalettes(&self);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AnyObject>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, new_delegate: Option<&AnyObject>);

        #[cfg(feature = "InputMethodKit_IMKServer")]
        #[method_id(@__retain_semantics Other server)]
        pub unsafe fn server(&self) -> Option<Id<IMKServer>>;

        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<TodoProtocols>>;

        #[method(inputControllerWillClose)]
        pub unsafe fn inputControllerWillClose(&self);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(annotationSelected:forCandidate:)]
        pub unsafe fn annotationSelected_forCandidate(
            &self,
            annotation_string: Option<&NSAttributedString>,
            candidate_string: Option<&NSAttributedString>,
        );

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(candidateSelectionChanged:)]
        pub unsafe fn candidateSelectionChanged(
            &self,
            candidate_string: Option<&NSAttributedString>,
        );

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(candidateSelected:)]
        pub unsafe fn candidateSelected(&self, candidate_string: Option<&NSAttributedString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "InputMethodKit_IMKInputController")]
    unsafe impl IMKInputController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
