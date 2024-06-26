//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_protocol!(
    pub unsafe trait ILMessageFilterCapabilitiesQueryHandling: NSObjectProtocol {
        #[cfg(all(
            feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryRequest",
            feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryResponse",
            feature = "IdentityLookup_ILMessageFilterExtensionContext"
        ))]
        #[method(handleCapabilitiesQueryRequest:context:completion:)]
        unsafe fn handleCapabilitiesQueryRequest_context_completion(
            &self,
            capabilities_query_request: &ILMessageFilterCapabilitiesQueryRequest,
            context: &ILMessageFilterExtensionContext,
            completion: &Block<(NonNull<ILMessageFilterCapabilitiesQueryResponse>,), ()>,
        );
    }

    unsafe impl ProtocolType for dyn ILMessageFilterCapabilitiesQueryHandling {}
);
