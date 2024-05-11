//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_static!(ASExtensionErrorDomain: Option<&'static NSErrorDomain>);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum ASExtensionErrorCode {
        ASExtensionErrorCodeFailed = 0,
        ASExtensionErrorCodeUserCanceled = 1,
        ASExtensionErrorCodeUserInteractionRequired = 100,
        ASExtensionErrorCodeCredentialIdentityNotFound = 101,
    }
);

extern_static!(ASExtensionLocalizedFailureReasonErrorKey: Option<&'static NSErrorUserInfoKey>);
