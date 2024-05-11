//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKRecordZoneID")]
    pub struct CKRecordZoneID;

    #[cfg(feature = "CloudKit_CKRecordZoneID")]
    unsafe impl ClassType for CKRecordZoneID {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKRecordZoneID")]
unsafe impl NSCoding for CKRecordZoneID {}

#[cfg(feature = "CloudKit_CKRecordZoneID")]
unsafe impl NSCopying for CKRecordZoneID {}

#[cfg(feature = "CloudKit_CKRecordZoneID")]
unsafe impl NSObjectProtocol for CKRecordZoneID {}

#[cfg(feature = "CloudKit_CKRecordZoneID")]
unsafe impl NSSecureCoding for CKRecordZoneID {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKRecordZoneID")]
    unsafe impl CKRecordZoneID {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithZoneName:ownerName:)]
        pub unsafe fn initWithZoneName_ownerName(
            this: Option<Allocated<Self>>,
            zone_name: &NSString,
            owner_name: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other zoneName)]
        pub unsafe fn zoneName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ownerName)]
        pub unsafe fn ownerName(&self) -> Id<NSString>;
    }
);