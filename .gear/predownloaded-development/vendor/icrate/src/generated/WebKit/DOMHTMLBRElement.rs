//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLBRElement")]
    #[deprecated]
    pub struct DOMHTMLBRElement;

    #[cfg(feature = "WebKit_DOMHTMLBRElement")]
    unsafe impl ClassType for DOMHTMLBRElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLBRElement")]
unsafe impl DOMEventTarget for DOMHTMLBRElement {}

#[cfg(feature = "WebKit_DOMHTMLBRElement")]
unsafe impl NSCopying for DOMHTMLBRElement {}

#[cfg(feature = "WebKit_DOMHTMLBRElement")]
unsafe impl NSObjectProtocol for DOMHTMLBRElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLBRElement")]
    unsafe impl DOMHTMLBRElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other clear)]
        pub unsafe fn clear(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setClear:)]
        pub unsafe fn setClear(&self, clear: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLBRElement")]
    unsafe impl DOMHTMLBRElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLBRElement")]
    unsafe impl DOMHTMLBRElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);