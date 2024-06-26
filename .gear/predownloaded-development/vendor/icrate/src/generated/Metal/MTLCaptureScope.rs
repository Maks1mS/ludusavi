//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_protocol!(
    pub unsafe trait MTLCaptureScope: NSObjectProtocol {
        #[method(beginScope)]
        fn beginScope(&self);

        #[method(endScope)]
        fn endScope(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other commandQueue)]
        unsafe fn commandQueue(&self) -> Option<Id<ProtocolObject<dyn MTLCommandQueue>>>;
    }

    unsafe impl ProtocolType for dyn MTLCaptureScope {}
);
