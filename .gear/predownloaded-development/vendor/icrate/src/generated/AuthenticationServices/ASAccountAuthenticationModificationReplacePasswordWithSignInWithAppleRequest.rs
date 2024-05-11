//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(
        feature = "AuthenticationServices_ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest"
    )]
    pub struct ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest;

    #[cfg(
        feature = "AuthenticationServices_ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest"
    )]
    unsafe impl ClassType
        for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest
    {
        #[inherits(NSObject)]
        type Super = ASAccountAuthenticationModificationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(
    feature = "AuthenticationServices_ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest"
)]
unsafe impl NSObjectProtocol
    for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest
{
}

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest"
    )]
    unsafe impl ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest {
        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithUser:serviceIdentifier:userInfo:)]
        pub unsafe fn initWithUser_serviceIdentifier_userInfo(
            this: Option<Allocated<Self>>,
            user: &NSString,
            service_identifier: &ASCredentialServiceIdentifier,
            user_info: Option<&NSDictionary>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Id<NSString>;

        #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Other serviceIdentifier)]
        pub unsafe fn serviceIdentifier(&self) -> Id<ASCredentialServiceIdentifier>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(
        feature = "AuthenticationServices_ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest"
    )]
    unsafe impl ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);