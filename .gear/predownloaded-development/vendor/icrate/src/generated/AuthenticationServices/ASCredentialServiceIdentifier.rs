//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASCredentialServiceIdentifierType {
        ASCredentialServiceIdentifierTypeDomain = 0,
        ASCredentialServiceIdentifierTypeURL = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
    pub struct ASCredentialServiceIdentifier;

    #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
    unsafe impl ClassType for ASCredentialServiceIdentifier {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
unsafe impl NSCoding for ASCredentialServiceIdentifier {}

#[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
unsafe impl NSCopying for ASCredentialServiceIdentifier {}

#[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
unsafe impl NSObjectProtocol for ASCredentialServiceIdentifier {}

#[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
unsafe impl NSSecureCoding for ASCredentialServiceIdentifier {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
    unsafe impl ASCredentialServiceIdentifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:type:)]
        pub unsafe fn initWithIdentifier_type(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            r#type: ASCredentialServiceIdentifierType,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> ASCredentialServiceIdentifierType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
    unsafe impl ASCredentialServiceIdentifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);