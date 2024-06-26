//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        #[deprecated]
        NSNoInterfaceStyle = 0,
        #[deprecated]
        NSNextStepInterfaceStyle = 1,
        #[deprecated]
        NSWindows95InterfaceStyle = 2,
        #[deprecated]
        NSMacintoshInterfaceStyle = 3,
    }
);

pub type NSInterfaceStyle = NSUInteger;

extern_fn!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "Foundation_NSString"))]
    #[deprecated]
    pub unsafe fn NSInterfaceStyleForKey(
        key: Option<&NSString>,
        responder: Option<&NSResponder>,
    ) -> NSInterfaceStyle;
);

extern_methods!(
    /// NSInterfaceStyle
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[deprecated]
        #[method(interfaceStyle)]
        pub unsafe fn interfaceStyle(&self) -> NSInterfaceStyle;

        #[deprecated]
        #[method(setInterfaceStyle:)]
        pub unsafe fn setInterfaceStyle(&self, interface_style: NSInterfaceStyle);
    }
);

extern_static!(NSInterfaceStyleDefault: Option<&'static NSString>);
