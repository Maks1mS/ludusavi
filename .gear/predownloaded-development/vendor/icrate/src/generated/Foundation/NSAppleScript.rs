//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSAppleScriptErrorMessage: &'static NSString);

extern_static!(NSAppleScriptErrorNumber: &'static NSString);

extern_static!(NSAppleScriptErrorAppName: &'static NSString);

extern_static!(NSAppleScriptErrorBriefMessage: &'static NSString);

extern_static!(NSAppleScriptErrorRange: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSAppleScript")]
    pub struct NSAppleScript;

    #[cfg(feature = "Foundation_NSAppleScript")]
    unsafe impl ClassType for NSAppleScript {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSAppleScript")]
unsafe impl NSCopying for NSAppleScript {}

#[cfg(feature = "Foundation_NSAppleScript")]
unsafe impl NSObjectProtocol for NSAppleScript {}

extern_methods!(
    #[cfg(feature = "Foundation_NSAppleScript")]
    unsafe impl NSAppleScript {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, AnyObject>>>>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithSource:)]
        pub unsafe fn initWithSource(
            this: Option<Allocated<Self>>,
            source: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Option<Id<NSString>>;

        #[method(isCompiled)]
        pub unsafe fn isCompiled(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(compileAndReturnError:)]
        pub unsafe fn compileAndReturnError(
            &self,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, AnyObject>>>>,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other executeAndReturnError:)]
        pub unsafe fn executeAndReturnError(
            &self,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, AnyObject>>>>,
        ) -> Id<NSAppleEventDescriptor>;

        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other executeAppleEvent:error:)]
        pub unsafe fn executeAppleEvent_error(
            &self,
            event: &NSAppleEventDescriptor,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, AnyObject>>>>,
        ) -> Id<NSAppleEventDescriptor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSAppleScript")]
    unsafe impl NSAppleScript {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);