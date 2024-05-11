//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMBlob")]
    #[deprecated]
    pub struct DOMBlob;

    #[cfg(feature = "WebKit_DOMBlob")]
    unsafe impl ClassType for DOMBlob {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMBlob")]
unsafe impl NSCopying for DOMBlob {}

#[cfg(feature = "WebKit_DOMBlob")]
unsafe impl NSObjectProtocol for DOMBlob {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMBlob")]
    unsafe impl DOMBlob {
        #[method(size)]
        pub unsafe fn size(&self) -> c_ulonglong;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMBlob")]
    unsafe impl DOMBlob {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMBlob")]
    unsafe impl DOMBlob {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);