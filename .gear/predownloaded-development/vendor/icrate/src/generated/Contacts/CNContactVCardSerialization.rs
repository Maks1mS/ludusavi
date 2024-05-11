//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNContactVCardSerialization")]
    pub struct CNContactVCardSerialization;

    #[cfg(feature = "Contacts_CNContactVCardSerialization")]
    unsafe impl ClassType for CNContactVCardSerialization {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Contacts_CNContactVCardSerialization")]
unsafe impl NSObjectProtocol for CNContactVCardSerialization {}

extern_methods!(
    #[cfg(feature = "Contacts_CNContactVCardSerialization")]
    unsafe impl CNContactVCardSerialization {
        #[method_id(@__retain_semantics Other descriptorForRequiredKeys)]
        pub unsafe fn descriptorForRequiredKeys() -> Id<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other dataWithContacts:error:_)]
        pub unsafe fn dataWithContacts_error(
            contacts: &NSArray<CNContact>,
        ) -> Result<Id<NSData>, Id<NSError>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other contactsWithData:error:_)]
        pub unsafe fn contactsWithData_error(
            data: &NSData,
        ) -> Result<Id<NSArray<CNContact>>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Contacts_CNContactVCardSerialization")]
    unsafe impl CNContactVCardSerialization {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);