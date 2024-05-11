//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCSSRuleList")]
    #[deprecated]
    pub struct DOMCSSRuleList;

    #[cfg(feature = "WebKit_DOMCSSRuleList")]
    unsafe impl ClassType for DOMCSSRuleList {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMCSSRuleList")]
unsafe impl NSCopying for DOMCSSRuleList {}

#[cfg(feature = "WebKit_DOMCSSRuleList")]
unsafe impl NSObjectProtocol for DOMCSSRuleList {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMCSSRuleList")]
    unsafe impl DOMCSSRuleList {
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[cfg(feature = "WebKit_DOMCSSRule")]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMCSSRule>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMCSSRuleList")]
    unsafe impl DOMCSSRuleList {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMCSSRuleList")]
    unsafe impl DOMCSSRuleList {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
