//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventRequest")]
    pub struct NSPersistentCloudKitContainerEventRequest;

    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventRequest")]
    unsafe impl ClassType for NSPersistentCloudKitContainerEventRequest {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventRequest")]
unsafe impl NSCopying for NSPersistentCloudKitContainerEventRequest {}

#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventRequest")]
unsafe impl NSObjectProtocol for NSPersistentCloudKitContainerEventRequest {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventRequest")]
    unsafe impl NSPersistentCloudKitContainerEventRequest {
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentCloudKitContainerEventResultType;

        #[method(setResultType:)]
        pub unsafe fn setResultType(
            &self,
            result_type: NSPersistentCloudKitContainerEventResultType,
        );

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other fetchEventsAfterDate:)]
        pub unsafe fn fetchEventsAfterDate(date: &NSDate) -> Id<Self>;

        #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEvent")]
        #[method_id(@__retain_semantics Other fetchEventsAfterEvent:)]
        pub unsafe fn fetchEventsAfterEvent(
            event: Option<&NSPersistentCloudKitContainerEvent>,
        ) -> Id<Self>;

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchEventsMatchingFetchRequest:)]
        pub unsafe fn fetchEventsMatchingFetchRequest(fetch_request: &NSFetchRequest) -> Id<Self>;

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequestForEvents)]
        pub unsafe fn fetchRequestForEvents() -> Id<NSFetchRequest>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventRequest")]
    unsafe impl NSPersistentCloudKitContainerEventRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);