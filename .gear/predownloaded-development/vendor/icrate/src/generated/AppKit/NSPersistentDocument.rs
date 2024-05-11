//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPersistentDocument")]
    pub struct NSPersistentDocument;

    #[cfg(feature = "AppKit_NSPersistentDocument")]
    unsafe impl ClassType for NSPersistentDocument {
        #[inherits(NSObject)]
        type Super = NSDocument;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSPersistentDocument")]
unsafe impl NSEditorRegistration for NSPersistentDocument {}

#[cfg(feature = "AppKit_NSPersistentDocument")]
unsafe impl NSFilePresenter for NSPersistentDocument {}

#[cfg(feature = "AppKit_NSPersistentDocument")]
unsafe impl NSMenuItemValidation for NSPersistentDocument {}

#[cfg(feature = "AppKit_NSPersistentDocument")]
unsafe impl NSObjectProtocol for NSPersistentDocument {}

#[cfg(feature = "AppKit_NSPersistentDocument")]
unsafe impl NSUserInterfaceValidations for NSPersistentDocument {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPersistentDocument")]
    unsafe impl NSPersistentDocument {
        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Option<Id<NSManagedObjectContext>>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method(setManagedObjectContext:)]
        pub unsafe fn setManagedObjectContext(
            &self,
            managed_object_context: Option<&NSManagedObjectContext>,
        );

        #[cfg(feature = "CoreData_NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Option<Id<NSManagedObjectModel>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(configurePersistentStoreCoordinatorForURL:ofType:modelConfiguration:storeOptions:error:_)]
        pub unsafe fn configurePersistentStoreCoordinatorForURL_ofType_modelConfiguration_storeOptions_error(
            &self,
            url: &NSURL,
            file_type: &NSString,
            configuration: Option<&NSString>,
            store_options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other persistentStoreTypeForFileType:)]
        pub unsafe fn persistentStoreTypeForFileType(&self, file_type: &NSString) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(writeToURL:ofType:forSaveOperation:originalContentsURL:error:_)]
        pub unsafe fn writeToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            absolute_url: &NSURL,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
            absolute_original_contents_url: Option<&NSURL>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(readFromURL:ofType:error:_)]
        pub unsafe fn readFromURL_ofType_error(
            &self,
            absolute_url: &NSURL,
            type_name: &NSString,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(revertToContentsOfURL:ofType:error:_)]
        pub unsafe fn revertToContentsOfURL_ofType_error(
            &self,
            in_absolute_url: &NSURL,
            in_type_name: &NSString,
        ) -> Result<(), Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDocument`
    #[cfg(feature = "AppKit_NSPersistentDocument")]
    unsafe impl NSPersistentDocument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithType:error:_)]
        pub unsafe fn initWithType_error(
            this: Option<Allocated<Self>>,
            type_name: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:ofType:error:_)]
        pub unsafe fn initWithContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            type_name: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initForURL:withContentsOfURL:ofType:error:_)]
        pub unsafe fn initForURL_withContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            url_or_nil: Option<&NSURL>,
            contents_url: &NSURL,
            type_name: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSPersistentDocument")]
    unsafe impl NSPersistentDocument {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSPersistentDocument")]
    unsafe impl NSPersistentDocument {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[deprecated]
        #[method(configurePersistentStoreCoordinatorForURL:ofType:error:_)]
        pub unsafe fn configurePersistentStoreCoordinatorForURL_ofType_error(
            &self,
            url: Option<&NSURL>,
            file_type: Option<&NSString>,
        ) -> Result<(), Id<NSError>>;
    }
);