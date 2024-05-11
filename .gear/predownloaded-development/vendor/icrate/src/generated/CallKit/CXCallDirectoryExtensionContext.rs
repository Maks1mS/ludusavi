//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait CXCallDirectoryExtensionContextDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "CallKit_CXCallDirectoryExtensionContext",
            feature = "Foundation_NSError"
        ))]
        #[method(requestFailedForExtensionContext:withError:)]
        unsafe fn requestFailedForExtensionContext_withError(
            &self,
            extension_context: &CXCallDirectoryExtensionContext,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn CXCallDirectoryExtensionContextDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
    pub struct CXCallDirectoryExtensionContext;

    #[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
    unsafe impl ClassType for CXCallDirectoryExtensionContext {
        #[inherits(NSObject)]
        type Super = NSExtensionContext;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
unsafe impl NSObjectProtocol for CXCallDirectoryExtensionContext {}

extern_methods!(
    #[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
    unsafe impl CXCallDirectoryExtensionContext {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn CXCallDirectoryExtensionContextDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CXCallDirectoryExtensionContextDelegate>>,
        );

        #[method(isIncremental)]
        pub unsafe fn isIncremental(&self) -> bool;

        #[method(addBlockingEntryWithNextSequentialPhoneNumber:)]
        pub unsafe fn addBlockingEntryWithNextSequentialPhoneNumber(
            &self,
            phone_number: CXCallDirectoryPhoneNumber,
        );

        #[method(removeBlockingEntryWithPhoneNumber:)]
        pub unsafe fn removeBlockingEntryWithPhoneNumber(
            &self,
            phone_number: CXCallDirectoryPhoneNumber,
        );

        #[method(removeAllBlockingEntries)]
        pub unsafe fn removeAllBlockingEntries(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(addIdentificationEntryWithNextSequentialPhoneNumber:label:)]
        pub unsafe fn addIdentificationEntryWithNextSequentialPhoneNumber_label(
            &self,
            phone_number: CXCallDirectoryPhoneNumber,
            label: &NSString,
        );

        #[method(removeIdentificationEntryWithPhoneNumber:)]
        pub unsafe fn removeIdentificationEntryWithPhoneNumber(
            &self,
            phone_number: CXCallDirectoryPhoneNumber,
        );

        #[method(removeAllIdentificationEntries)]
        pub unsafe fn removeAllIdentificationEntries(&self);

        #[method(completeRequestWithCompletionHandler:)]
        pub unsafe fn completeRequestWithCompletionHandler(
            &self,
            completion: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(completeRequestReturningItems:completionHandler:)]
        pub unsafe fn completeRequestReturningItems_completionHandler(
            &self,
            items: Option<&NSArray>,
            completion_handler: Option<&Block<(Bool,), ()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
    unsafe impl CXCallDirectoryExtensionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
