//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(WebKitErrorDomain: Option<&'static NSString>);

extern_static!(WebKitErrorMIMETypeKey: Option<&'static NSString>);

extern_static!(WebKitErrorPlugInNameKey: Option<&'static NSString>);

extern_static!(WebKitErrorPlugInPageURLStringKey: Option<&'static NSString>);

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        WebKitErrorCannotShowMIMEType = 100,
        #[deprecated]
        WebKitErrorCannotShowURL = 101,
        #[deprecated]
        WebKitErrorFrameLoadInterruptedByPolicyChange = 102,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        WebKitErrorCannotFindPlugIn = 200,
        #[deprecated]
        WebKitErrorCannotLoadPlugIn = 201,
        #[deprecated]
        WebKitErrorJavaUnavailable = 202,
        #[deprecated]
        WebKitErrorBlockedPlugInVersion = 203,
    }
);
