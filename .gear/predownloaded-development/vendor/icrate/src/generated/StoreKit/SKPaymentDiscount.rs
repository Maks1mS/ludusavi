//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKPaymentDiscount")]
    pub struct SKPaymentDiscount;

    #[cfg(feature = "StoreKit_SKPaymentDiscount")]
    unsafe impl ClassType for SKPaymentDiscount {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKPaymentDiscount")]
unsafe impl NSObjectProtocol for SKPaymentDiscount {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKPaymentDiscount")]
    unsafe impl SKPaymentDiscount {
        #[cfg(all(
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString",
            feature = "Foundation_NSUUID"
        ))]
        #[method_id(@__retain_semantics Init initWithIdentifier:keyIdentifier:nonce:signature:timestamp:)]
        pub unsafe fn initWithIdentifier_keyIdentifier_nonce_signature_timestamp(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            key_identifier: &NSString,
            nonce: &NSUUID,
            signature: &NSString,
            timestamp: &NSNumber,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other keyIdentifier)]
        pub unsafe fn keyIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other nonce)]
        pub unsafe fn nonce(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other signature)]
        pub unsafe fn signature(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Id<NSNumber>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKPaymentDiscount")]
    unsafe impl SKPaymentDiscount {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);