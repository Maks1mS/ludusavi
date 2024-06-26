//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_protocol!(
    pub unsafe trait ILMessageFilterQueryHandling: NSObjectProtocol {
        #[cfg(all(
            feature = "IdentityLookup_ILMessageFilterExtensionContext",
            feature = "IdentityLookup_ILMessageFilterQueryRequest",
            feature = "IdentityLookup_ILMessageFilterQueryResponse"
        ))]
        #[method(handleQueryRequest:context:completion:)]
        unsafe fn handleQueryRequest_context_completion(
            &self,
            query_request: &ILMessageFilterQueryRequest,
            context: &ILMessageFilterExtensionContext,
            completion: &Block<(NonNull<ILMessageFilterQueryResponse>,), ()>,
        );
    }

    unsafe impl ProtocolType for dyn ILMessageFilterQueryHandling {}
);
