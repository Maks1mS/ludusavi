//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_protocol!(
    pub unsafe trait GKViewController {}

    unsafe impl ProtocolType for dyn GKViewController {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKDialogController")]
    pub struct GKDialogController;

    #[cfg(feature = "GameKit_GKDialogController")]
    unsafe impl ClassType for GKDialogController {
        #[inherits(NSObject)]
        type Super = NSResponder;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKDialogController")]
unsafe impl NSCoding for GKDialogController {}

#[cfg(feature = "GameKit_GKDialogController")]
unsafe impl NSObjectProtocol for GKDialogController {}

extern_methods!(
    #[cfg(feature = "GameKit_GKDialogController")]
    unsafe impl GKDialogController {
        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other parentWindow)]
        pub unsafe fn parentWindow(&self) -> Option<Id<NSWindow>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parent_window: Option<&NSWindow>);

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(presentViewController:)]
        pub unsafe fn presentViewController(&self, view_controller: &NSViewController) -> bool;

        #[method(dismiss:)]
        pub unsafe fn dismiss(&self, sender: &AnyObject);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "GameKit_GKDialogController")]
    unsafe impl GKDialogController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKDialogController")]
    unsafe impl GKDialogController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// SharedDialogController
    #[cfg(feature = "GameKit_GKDialogController")]
    unsafe impl GKDialogController {
        #[method_id(@__retain_semantics Other sharedDialogController)]
        pub unsafe fn sharedDialogController() -> Id<GKDialogController>;
    }
);