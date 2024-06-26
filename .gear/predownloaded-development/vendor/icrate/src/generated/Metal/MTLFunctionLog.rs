//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLFunctionLogType {
        MTLFunctionLogTypeValidation = 0,
    }
);

extern_protocol!(
    pub unsafe trait MTLLogContainer: NSFastEnumeration {}

    unsafe impl ProtocolType for dyn MTLLogContainer {}
);

extern_protocol!(
    pub unsafe trait MTLFunctionLogDebugLocation: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other functionName)]
        unsafe fn functionName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[method(line)]
        unsafe fn line(&self) -> NSUInteger;

        #[method(column)]
        unsafe fn column(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn MTLFunctionLogDebugLocation {}
);

extern_protocol!(
    pub unsafe trait MTLFunctionLog: NSObjectProtocol {
        #[method(type)]
        unsafe fn r#type(&self) -> MTLFunctionLogType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other encoderLabel)]
        unsafe fn encoderLabel(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other function)]
        unsafe fn function(&self) -> Option<Id<ProtocolObject<dyn MTLFunction>>>;

        #[method_id(@__retain_semantics Other debugLocation)]
        unsafe fn debugLocation(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLFunctionLogDebugLocation>>>;
    }

    unsafe impl ProtocolType for dyn MTLFunctionLog {}
);
