//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(WebPlugInBaseURLKey: Option<&'static NSString>);

extern_static!(WebPlugInAttributesKey: Option<&'static NSString>);

extern_static!(WebPlugInContainerKey: Option<&'static NSString>);

extern_static!(WebPlugInContainingElementKey: Option<&'static NSString>);

extern_static!(WebPlugInShouldLoadMainResourceKey: Option<&'static NSString>);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebPlugInViewFactory: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Other plugInViewWithArguments:)]
        unsafe fn plugInViewWithArguments(arguments: Option<&NSDictionary>) -> Option<Id<NSView>>;
    }

    unsafe impl ProtocolType for dyn WebPlugInViewFactory {}
);