//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKContentWorld")]
    pub struct WKContentWorld;

    #[cfg(feature = "WebKit_WKContentWorld")]
    unsafe impl ClassType for WKContentWorld {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WKContentWorld")]
unsafe impl NSObjectProtocol for WKContentWorld {}

extern_methods!(
    #[cfg(feature = "WebKit_WKContentWorld")]
    unsafe impl WKContentWorld {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Other pageWorld)]
        pub unsafe fn pageWorld() -> Id<WKContentWorld>;

        #[method_id(@__retain_semantics Other defaultClientWorld)]
        pub unsafe fn defaultClientWorld() -> Id<WKContentWorld>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other worldWithName:)]
        pub unsafe fn worldWithName(name: &NSString) -> Id<WKContentWorld>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;
    }
);