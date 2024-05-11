//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
    pub struct ASAuthorizationPlatformPublicKeyCredentialDescriptor;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
unsafe impl ASAuthorizationPublicKeyCredentialDescriptor
    for ASAuthorizationPlatformPublicKeyCredentialDescriptor
{
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
unsafe impl NSCoding for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
unsafe impl NSCopying for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
unsafe impl NSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialDescriptor {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialDescriptor {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithCredentialID:)]
        pub unsafe fn initWithCredentialID(
            this: Option<Allocated<Self>>,
            credential_id: &NSData,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);