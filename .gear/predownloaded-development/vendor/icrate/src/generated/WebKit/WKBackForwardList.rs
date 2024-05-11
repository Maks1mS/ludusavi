//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKBackForwardList")]
    pub struct WKBackForwardList;

    #[cfg(feature = "WebKit_WKBackForwardList")]
    unsafe impl ClassType for WKBackForwardList {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WKBackForwardList")]
unsafe impl NSObjectProtocol for WKBackForwardList {}

extern_methods!(
    #[cfg(feature = "WebKit_WKBackForwardList")]
    unsafe impl WKBackForwardList {
        #[cfg(feature = "WebKit_WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other currentItem)]
        pub unsafe fn currentItem(&self) -> Option<Id<WKBackForwardListItem>>;

        #[cfg(feature = "WebKit_WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other backItem)]
        pub unsafe fn backItem(&self) -> Option<Id<WKBackForwardListItem>>;

        #[cfg(feature = "WebKit_WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other forwardItem)]
        pub unsafe fn forwardItem(&self) -> Option<Id<WKBackForwardListItem>>;

        #[cfg(feature = "WebKit_WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: NSInteger) -> Option<Id<WKBackForwardListItem>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "WebKit_WKBackForwardListItem"
        ))]
        #[method_id(@__retain_semantics Other backList)]
        pub unsafe fn backList(&self) -> Id<NSArray<WKBackForwardListItem>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "WebKit_WKBackForwardListItem"
        ))]
        #[method_id(@__retain_semantics Other forwardList)]
        pub unsafe fn forwardList(&self) -> Id<NSArray<WKBackForwardListItem>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WKBackForwardList")]
    unsafe impl WKBackForwardList {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
