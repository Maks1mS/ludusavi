//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNFetchRequest")]
    pub struct CNFetchRequest;

    #[cfg(feature = "Contacts_CNFetchRequest")]
    unsafe impl ClassType for CNFetchRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Contacts_CNFetchRequest")]
unsafe impl NSObjectProtocol for CNFetchRequest {}

extern_methods!(
    #[cfg(feature = "Contacts_CNFetchRequest")]
    unsafe impl CNFetchRequest {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Contacts_CNFetchRequest")]
    unsafe impl CNFetchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);