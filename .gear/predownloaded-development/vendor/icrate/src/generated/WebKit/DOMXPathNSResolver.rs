//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_protocol!(
    #[deprecated]
    pub unsafe trait DOMXPathNSResolver: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other lookupNamespaceURI:)]
        unsafe fn lookupNamespaceURI(&self, prefix: Option<&NSString>) -> Option<Id<NSString>>;
    }

    unsafe impl ProtocolType for dyn DOMXPathNSResolver {}
);