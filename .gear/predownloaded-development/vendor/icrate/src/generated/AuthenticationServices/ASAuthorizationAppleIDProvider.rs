//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASAuthorizationAppleIDProviderCredentialState {
        ASAuthorizationAppleIDProviderCredentialRevoked = 0,
        ASAuthorizationAppleIDProviderCredentialAuthorized = 1,
        ASAuthorizationAppleIDProviderCredentialNotFound = 2,
        ASAuthorizationAppleIDProviderCredentialTransferred = 3,
    }
);

extern_static!(
    ASAuthorizationAppleIDProviderCredentialRevokedNotification: &'static NSNotificationName
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDProvider")]
    pub struct ASAuthorizationAppleIDProvider;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDProvider")]
    unsafe impl ClassType for ASAuthorizationAppleIDProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDProvider")]
unsafe impl ASAuthorizationProvider for ASAuthorizationAppleIDProvider {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDProvider")]
unsafe impl NSObjectProtocol for ASAuthorizationAppleIDProvider {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDProvider")]
    unsafe impl ASAuthorizationAppleIDProvider {
        #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDRequest")]
        #[method_id(@__retain_semantics Other createRequest)]
        pub unsafe fn createRequest(&self) -> Id<ASAuthorizationAppleIDRequest>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(getCredentialStateForUserID:completion:)]
        pub unsafe fn getCredentialStateForUserID_completion(
            &self,
            user_id: &NSString,
            completion: &Block<(ASAuthorizationAppleIDProviderCredentialState, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDProvider")]
    unsafe impl ASAuthorizationAppleIDProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);