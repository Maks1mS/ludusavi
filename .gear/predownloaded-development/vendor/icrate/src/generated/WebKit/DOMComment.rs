//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMComment")]
    #[deprecated]
    pub struct DOMComment;

    #[cfg(feature = "WebKit_DOMComment")]
    unsafe impl ClassType for DOMComment {
        #[inherits(DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCharacterData;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMComment")]
unsafe impl DOMEventTarget for DOMComment {}

#[cfg(feature = "WebKit_DOMComment")]
unsafe impl NSCopying for DOMComment {}

#[cfg(feature = "WebKit_DOMComment")]
unsafe impl NSObjectProtocol for DOMComment {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMComment")]
    unsafe impl DOMComment {}
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMComment")]
    unsafe impl DOMComment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMComment")]
    unsafe impl DOMComment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);