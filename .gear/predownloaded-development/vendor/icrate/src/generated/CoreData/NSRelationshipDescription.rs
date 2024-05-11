//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDeleteRule {
        NSNoActionDeleteRule = 0,
        NSNullifyDeleteRule = 1,
        NSCascadeDeleteRule = 2,
        NSDenyDeleteRule = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSRelationshipDescription")]
    pub struct NSRelationshipDescription;

    #[cfg(feature = "CoreData_NSRelationshipDescription")]
    unsafe impl ClassType for NSRelationshipDescription {
        #[inherits(NSObject)]
        type Super = NSPropertyDescription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSRelationshipDescription")]
unsafe impl NSCoding for NSRelationshipDescription {}

#[cfg(feature = "CoreData_NSRelationshipDescription")]
unsafe impl NSCopying for NSRelationshipDescription {}

#[cfg(feature = "CoreData_NSRelationshipDescription")]
unsafe impl NSObjectProtocol for NSRelationshipDescription {}

extern_methods!(
    #[cfg(feature = "CoreData_NSRelationshipDescription")]
    unsafe impl NSRelationshipDescription {
        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other destinationEntity)]
        pub unsafe fn destinationEntity(&self) -> Option<Id<NSEntityDescription>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method(setDestinationEntity:)]
        pub unsafe fn setDestinationEntity(&self, destination_entity: Option<&NSEntityDescription>);

        #[method_id(@__retain_semantics Other inverseRelationship)]
        pub unsafe fn inverseRelationship(&self) -> Option<Id<NSRelationshipDescription>>;

        #[method(setInverseRelationship:)]
        pub unsafe fn setInverseRelationship(
            &self,
            inverse_relationship: Option<&NSRelationshipDescription>,
        );

        #[method(maxCount)]
        pub unsafe fn maxCount(&self) -> NSUInteger;

        #[method(setMaxCount:)]
        pub unsafe fn setMaxCount(&self, max_count: NSUInteger);

        #[method(minCount)]
        pub unsafe fn minCount(&self) -> NSUInteger;

        #[method(setMinCount:)]
        pub unsafe fn setMinCount(&self, min_count: NSUInteger);

        #[method(deleteRule)]
        pub unsafe fn deleteRule(&self) -> NSDeleteRule;

        #[method(setDeleteRule:)]
        pub unsafe fn setDeleteRule(&self, delete_rule: NSDeleteRule);

        #[method(isToMany)]
        pub unsafe fn isToMany(&self) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other versionHash)]
        pub unsafe fn versionHash(&self) -> Id<NSData>;

        #[method(isOrdered)]
        pub unsafe fn isOrdered(&self) -> bool;

        #[method(setOrdered:)]
        pub unsafe fn setOrdered(&self, ordered: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSRelationshipDescription")]
    unsafe impl NSRelationshipDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);