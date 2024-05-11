//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCSSFontFaceRule")]
    #[deprecated]
    pub struct DOMCSSFontFaceRule;

    #[cfg(feature = "WebKit_DOMCSSFontFaceRule")]
    unsafe impl ClassType for DOMCSSFontFaceRule {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCSSRule;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMCSSFontFaceRule")]
unsafe impl NSCopying for DOMCSSFontFaceRule {}

#[cfg(feature = "WebKit_DOMCSSFontFaceRule")]
unsafe impl NSObjectProtocol for DOMCSSFontFaceRule {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMCSSFontFaceRule")]
    unsafe impl DOMCSSFontFaceRule {
        #[cfg(feature = "WebKit_DOMCSSStyleDeclaration")]
        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Id<DOMCSSStyleDeclaration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMCSSFontFaceRule")]
    unsafe impl DOMCSSFontFaceRule {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMCSSFontFaceRule")]
    unsafe impl DOMCSSFontFaceRule {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);