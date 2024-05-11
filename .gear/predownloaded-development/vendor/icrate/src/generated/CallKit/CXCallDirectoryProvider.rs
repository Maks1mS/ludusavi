//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXCallDirectoryProvider")]
    pub struct CXCallDirectoryProvider;

    #[cfg(feature = "CallKit_CXCallDirectoryProvider")]
    unsafe impl ClassType for CXCallDirectoryProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXCallDirectoryProvider")]
unsafe impl NSExtensionRequestHandling for CXCallDirectoryProvider {}

#[cfg(feature = "CallKit_CXCallDirectoryProvider")]
unsafe impl NSObjectProtocol for CXCallDirectoryProvider {}

extern_methods!(
    #[cfg(feature = "CallKit_CXCallDirectoryProvider")]
    unsafe impl CXCallDirectoryProvider {
        #[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
        #[method(beginRequestWithExtensionContext:)]
        pub unsafe fn beginRequestWithExtensionContext(
            &self,
            context: &CXCallDirectoryExtensionContext,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CallKit_CXCallDirectoryProvider")]
    unsafe impl CXCallDirectoryProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);