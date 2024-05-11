//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest"
    )]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest;

    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest"
    )]
    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest"
)]
unsafe impl ASAuthorizationPublicKeyCredentialRegistrationRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
{
}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest"
)]
unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest"
)]
unsafe impl NSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest"
)]
unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest"
)]
unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest"
    )]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other credentialParameters)]
        pub unsafe fn credentialParameters(
            &self,
        ) -> Id<NSArray<ASAuthorizationPublicKeyCredentialParameters>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters",
            feature = "Foundation_NSArray"
        ))]
        #[method(setCredentialParameters:)]
        pub unsafe fn setCredentialParameters(
            &self,
            credential_parameters: &NSArray<ASAuthorizationPublicKeyCredentialParameters>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other excludedCredentials)]
        pub unsafe fn excludedCredentials(
            &self,
        ) -> Id<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method(setExcludedCredentials:)]
        pub unsafe fn setExcludedCredentials(
            &self,
            excluded_credentials: &NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>,
        );

        #[method_id(@__retain_semantics Other residentKeyPreference)]
        pub unsafe fn residentKeyPreference(
            &self,
        ) -> Id<ASAuthorizationPublicKeyCredentialResidentKeyPreference>;

        #[method(setResidentKeyPreference:)]
        pub unsafe fn setResidentKeyPreference(
            &self,
            resident_key_preference: &ASAuthorizationPublicKeyCredentialResidentKeyPreference,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
