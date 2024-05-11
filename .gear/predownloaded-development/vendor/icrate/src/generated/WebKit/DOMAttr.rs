//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMAttr")]
    #[deprecated]
    pub struct DOMAttr;

    #[cfg(feature = "WebKit_DOMAttr")]
    unsafe impl ClassType for DOMAttr {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMNode;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMAttr")]
unsafe impl DOMEventTarget for DOMAttr {}

#[cfg(feature = "WebKit_DOMAttr")]
unsafe impl NSCopying for DOMAttr {}

#[cfg(feature = "WebKit_DOMAttr")]
unsafe impl NSObjectProtocol for DOMAttr {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMAttr")]
    unsafe impl DOMAttr {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[method(specified)]
        pub unsafe fn specified(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMElement")]
        #[method_id(@__retain_semantics Other ownerElement)]
        pub unsafe fn ownerElement(&self) -> Option<Id<DOMElement>>;

        #[cfg(feature = "WebKit_DOMCSSStyleDeclaration")]
        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Id<DOMCSSStyleDeclaration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMAttr")]
    unsafe impl DOMAttr {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMAttr")]
    unsafe impl DOMAttr {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);