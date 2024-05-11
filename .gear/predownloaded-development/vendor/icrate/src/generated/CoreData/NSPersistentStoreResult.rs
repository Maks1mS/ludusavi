//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBatchInsertRequestResultType {
        NSBatchInsertRequestResultTypeStatusOnly = 0x0,
        NSBatchInsertRequestResultTypeObjectIDs = 0x1,
        NSBatchInsertRequestResultTypeCount = 0x2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBatchUpdateRequestResultType {
        NSStatusOnlyResultType = 0x0,
        NSUpdatedObjectIDsResultType = 0x1,
        NSUpdatedObjectsCountResultType = 0x2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBatchDeleteRequestResultType {
        NSBatchDeleteResultTypeStatusOnly = 0x0,
        NSBatchDeleteResultTypeObjectIDs = 0x1,
        NSBatchDeleteResultTypeCount = 0x2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPersistentHistoryResultType {
        NSPersistentHistoryResultTypeStatusOnly = 0x0,
        NSPersistentHistoryResultTypeObjectIDs = 0x1,
        NSPersistentHistoryResultTypeCount = 0x2,
        NSPersistentHistoryResultTypeTransactionsOnly = 0x3,
        NSPersistentHistoryResultTypeChangesOnly = 0x4,
        NSPersistentHistoryResultTypeTransactionsAndChanges = 0x5,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentStoreResult")]
    pub struct NSPersistentStoreResult;

    #[cfg(feature = "CoreData_NSPersistentStoreResult")]
    unsafe impl ClassType for NSPersistentStoreResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentStoreResult")]
unsafe impl NSObjectProtocol for NSPersistentStoreResult {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentStoreResult")]
    unsafe impl NSPersistentStoreResult {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSPersistentStoreResult")]
    unsafe impl NSPersistentStoreResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentStoreAsynchronousResult")]
    pub struct NSPersistentStoreAsynchronousResult;

    #[cfg(feature = "CoreData_NSPersistentStoreAsynchronousResult")]
    unsafe impl ClassType for NSPersistentStoreAsynchronousResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentStoreAsynchronousResult")]
unsafe impl NSObjectProtocol for NSPersistentStoreAsynchronousResult {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentStoreAsynchronousResult")]
    unsafe impl NSPersistentStoreAsynchronousResult {
        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Id<NSManagedObjectContext>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other operationError)]
        pub unsafe fn operationError(&self) -> Option<Id<NSError>>;

        #[cfg(feature = "Foundation_NSProgress")]
        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Option<Id<NSProgress>>;

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSPersistentStoreAsynchronousResult")]
    unsafe impl NSPersistentStoreAsynchronousResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSAsynchronousFetchResult")]
    pub struct NSAsynchronousFetchResult<ResultType: Message = AnyObject> {
        __superclass: NSPersistentStoreAsynchronousResult,
        _inner0: PhantomData<*mut ResultType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "CoreData_NSAsynchronousFetchResult")]
    unsafe impl<ResultType: Message> ClassType for NSAsynchronousFetchResult<ResultType> {
        #[inherits(NSPersistentStoreResult, NSObject)]
        type Super = NSPersistentStoreAsynchronousResult;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "CoreData_NSAsynchronousFetchResult")]
unsafe impl<ResultType: Message> NSObjectProtocol for NSAsynchronousFetchResult<ResultType> {}

extern_methods!(
    #[cfg(feature = "CoreData_NSAsynchronousFetchResult")]
    unsafe impl<ResultType: Message> NSAsynchronousFetchResult<ResultType> {
        #[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Id<NSAsynchronousFetchRequest<ResultType>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other finalResult)]
        pub unsafe fn finalResult(&self) -> Option<Id<NSArray<ResultType>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSAsynchronousFetchResult")]
    unsafe impl<ResultType: Message> NSAsynchronousFetchResult<ResultType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSBatchInsertResult")]
    pub struct NSBatchInsertResult;

    #[cfg(feature = "CoreData_NSBatchInsertResult")]
    unsafe impl ClassType for NSBatchInsertResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSBatchInsertResult")]
unsafe impl NSObjectProtocol for NSBatchInsertResult {}

extern_methods!(
    #[cfg(feature = "CoreData_NSBatchInsertResult")]
    unsafe impl NSBatchInsertResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchInsertRequestResultType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSBatchInsertResult")]
    unsafe impl NSBatchInsertResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSBatchUpdateResult")]
    pub struct NSBatchUpdateResult;

    #[cfg(feature = "CoreData_NSBatchUpdateResult")]
    unsafe impl ClassType for NSBatchUpdateResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSBatchUpdateResult")]
unsafe impl NSObjectProtocol for NSBatchUpdateResult {}

extern_methods!(
    #[cfg(feature = "CoreData_NSBatchUpdateResult")]
    unsafe impl NSBatchUpdateResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchUpdateRequestResultType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSBatchUpdateResult")]
    unsafe impl NSBatchUpdateResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSBatchDeleteResult")]
    pub struct NSBatchDeleteResult;

    #[cfg(feature = "CoreData_NSBatchDeleteResult")]
    unsafe impl ClassType for NSBatchDeleteResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSBatchDeleteResult")]
unsafe impl NSObjectProtocol for NSBatchDeleteResult {}

extern_methods!(
    #[cfg(feature = "CoreData_NSBatchDeleteResult")]
    unsafe impl NSBatchDeleteResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchDeleteRequestResultType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSBatchDeleteResult")]
    unsafe impl NSBatchDeleteResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentHistoryResult")]
    pub struct NSPersistentHistoryResult;

    #[cfg(feature = "CoreData_NSPersistentHistoryResult")]
    unsafe impl ClassType for NSPersistentHistoryResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentHistoryResult")]
unsafe impl NSObjectProtocol for NSPersistentHistoryResult {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentHistoryResult")]
    unsafe impl NSPersistentHistoryResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentHistoryResultType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSPersistentHistoryResult")]
    unsafe impl NSPersistentHistoryResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPersistentCloudKitContainerEventResultType {
        NSPersistentCloudKitContainerEventResultTypeEvents = 0,
        NSPersistentCloudKitContainerEventResultTypeCountEvents = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventResult")]
    pub struct NSPersistentCloudKitContainerEventResult;

    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventResult")]
    unsafe impl ClassType for NSPersistentCloudKitContainerEventResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventResult")]
unsafe impl NSObjectProtocol for NSPersistentCloudKitContainerEventResult {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventResult")]
    unsafe impl NSPersistentCloudKitContainerEventResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentCloudKitContainerEventResultType;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);