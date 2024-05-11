//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
    pub struct NSAtomicStoreCacheNode;

    #[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
    unsafe impl ClassType for NSAtomicStoreCacheNode {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
unsafe impl NSObjectProtocol for NSAtomicStoreCacheNode {}

extern_methods!(
    #[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
    unsafe impl NSAtomicStoreCacheNode {
        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Init initWithObjectID:)]
        pub unsafe fn initWithObjectID(
            this: Option<Allocated<Self>>,
            moid: &NSManagedObjectID,
        ) -> Id<Self>;

        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Other objectID)]
        pub unsafe fn objectID(&self) -> Id<NSManagedObjectID>;

        #[cfg(all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other propertyCache)]
        pub unsafe fn propertyCache(&self) -> Option<Id<NSMutableDictionary<NSString, AnyObject>>>;

        #[cfg(all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(setPropertyCache:)]
        pub unsafe fn setPropertyCache(
            &self,
            property_cache: Option<&NSMutableDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
    unsafe impl NSAtomicStoreCacheNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
