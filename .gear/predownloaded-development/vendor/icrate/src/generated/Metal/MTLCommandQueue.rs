//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_protocol!(
    pub unsafe trait MTLCommandQueue: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other commandBuffer)]
        fn commandBuffer(&self) -> Option<Id<ProtocolObject<dyn MTLCommandBuffer>>>;

        #[cfg(feature = "Metal_MTLCommandBufferDescriptor")]
        #[method_id(@__retain_semantics Other commandBufferWithDescriptor:)]
        unsafe fn commandBufferWithDescriptor(
            &self,
            descriptor: &MTLCommandBufferDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLCommandBuffer>>>;

        #[method_id(@__retain_semantics Other commandBufferWithUnretainedReferences)]
        unsafe fn commandBufferWithUnretainedReferences(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLCommandBuffer>>>;

        #[deprecated = "Use MTLCaptureScope instead"]
        #[method(insertDebugCaptureBoundary)]
        unsafe fn insertDebugCaptureBoundary(&self);
    }

    unsafe impl ProtocolType for dyn MTLCommandQueue {}
);