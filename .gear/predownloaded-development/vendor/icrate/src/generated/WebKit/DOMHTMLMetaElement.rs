//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLMetaElement")]
    #[deprecated]
    pub struct DOMHTMLMetaElement;

    #[cfg(feature = "WebKit_DOMHTMLMetaElement")]
    unsafe impl ClassType for DOMHTMLMetaElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLMetaElement")]
unsafe impl DOMEventTarget for DOMHTMLMetaElement {}

#[cfg(feature = "WebKit_DOMHTMLMetaElement")]
unsafe impl NSCopying for DOMHTMLMetaElement {}

#[cfg(feature = "WebKit_DOMHTMLMetaElement")]
unsafe impl NSObjectProtocol for DOMHTMLMetaElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLMetaElement")]
    unsafe impl DOMHTMLMetaElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other content)]
        pub unsafe fn content(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setContent:)]
        pub unsafe fn setContent(&self, content: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other httpEquiv)]
        pub unsafe fn httpEquiv(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHttpEquiv:)]
        pub unsafe fn setHttpEquiv(&self, http_equiv: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other scheme)]
        pub unsafe fn scheme(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setScheme:)]
        pub unsafe fn setScheme(&self, scheme: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLMetaElement")]
    unsafe impl DOMHTMLMetaElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLMetaElement")]
    unsafe impl DOMHTMLMetaElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);