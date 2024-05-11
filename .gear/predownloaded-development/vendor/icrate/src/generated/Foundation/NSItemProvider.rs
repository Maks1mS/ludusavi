//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSItemProviderRepresentationVisibility {
        NSItemProviderRepresentationVisibilityAll = 0,
        NSItemProviderRepresentationVisibilityTeam = 1,
        NSItemProviderRepresentationVisibilityGroup = 2,
        NSItemProviderRepresentationVisibilityOwnProcess = 3,
    }
);

ns_options!(
    #[underlying(NSInteger)]
    pub enum NSItemProviderFileOptions {
        NSItemProviderFileOptionOpenInPlace = 1,
    }
);

extern_protocol!(
    pub unsafe trait NSItemProviderWriting: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other writableTypeIdentifiersForItemProvider)]
        unsafe fn writableTypeIdentifiersForItemProvider_class() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other writableTypeIdentifiersForItemProvider)]
        unsafe fn writableTypeIdentifiersForItemProvider(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(itemProviderVisibilityForRepresentationWithTypeIdentifier:)]
        unsafe fn itemProviderVisibilityForRepresentationWithTypeIdentifier_class(
            type_identifier: &NSString,
        ) -> NSItemProviderRepresentationVisibility;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(itemProviderVisibilityForRepresentationWithTypeIdentifier:)]
        unsafe fn itemProviderVisibilityForRepresentationWithTypeIdentifier(
            &self,
            type_identifier: &NSString,
        ) -> NSItemProviderRepresentationVisibility;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other loadDataWithTypeIdentifier:forItemProviderCompletionHandler:)]
        unsafe fn loadDataWithTypeIdentifier_forItemProviderCompletionHandler(
            &self,
            type_identifier: &NSString,
            completion_handler: &Block<(*mut NSData, *mut NSError), ()>,
        ) -> Option<Id<NSProgress>>;
    }

    unsafe impl ProtocolType for dyn NSItemProviderWriting {}
);

extern_protocol!(
    pub unsafe trait NSItemProviderReading: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other readableTypeIdentifiersForItemProvider)]
        unsafe fn readableTypeIdentifiersForItemProvider() -> Id<NSArray<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other objectWithItemProviderData:typeIdentifier:error:_)]
        unsafe fn objectWithItemProviderData_typeIdentifier_error(
            data: &NSData,
            type_identifier: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;
    }

    unsafe impl ProtocolType for dyn NSItemProviderReading {}
);

pub type NSItemProviderCompletionHandler =
    *mut Block<(*mut ProtocolObject<dyn NSSecureCoding>, *mut NSError), ()>;

pub type NSItemProviderLoadHandler = *mut Block<
    (
        NSItemProviderCompletionHandler,
        *const AnyClass,
        *mut NSDictionary,
    ),
    (),
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSItemProvider")]
    pub struct NSItemProvider;

    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl ClassType for NSItemProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSItemProvider")]
unsafe impl NSCopying for NSItemProvider {}

#[cfg(feature = "Foundation_NSItemProvider")]
unsafe impl NSObjectProtocol for NSItemProvider {}

extern_methods!(
    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl NSItemProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString"
        ))]
        #[method(registerDataRepresentationForTypeIdentifier:visibility:loadHandler:)]
        pub unsafe fn registerDataRepresentationForTypeIdentifier_visibility_loadHandler(
            &self,
            type_identifier: &NSString,
            visibility: NSItemProviderRepresentationVisibility,
            load_handler: &Block<
                (NonNull<Block<(*mut NSData, *mut NSError), ()>>,),
                *mut NSProgress,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(registerFileRepresentationForTypeIdentifier:fileOptions:visibility:loadHandler:)]
        pub unsafe fn registerFileRepresentationForTypeIdentifier_fileOptions_visibility_loadHandler(
            &self,
            type_identifier: &NSString,
            file_options: NSItemProviderFileOptions,
            visibility: NSItemProviderRepresentationVisibility,
            load_handler: &Block<
                (NonNull<Block<(*mut NSURL, Bool, *mut NSError), ()>>,),
                *mut NSProgress,
            >,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other registeredTypeIdentifiers)]
        pub unsafe fn registeredTypeIdentifiers(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other registeredTypeIdentifiersWithFileOptions:)]
        pub unsafe fn registeredTypeIdentifiersWithFileOptions(
            &self,
            file_options: NSItemProviderFileOptions,
        ) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasItemConformingToTypeIdentifier:)]
        pub unsafe fn hasItemConformingToTypeIdentifier(&self, type_identifier: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasRepresentationConformingToTypeIdentifier:fileOptions:)]
        pub unsafe fn hasRepresentationConformingToTypeIdentifier_fileOptions(
            &self,
            type_identifier: &NSString,
            file_options: NSItemProviderFileOptions,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other loadDataRepresentationForTypeIdentifier:completionHandler:)]
        pub unsafe fn loadDataRepresentationForTypeIdentifier_completionHandler(
            &self,
            type_identifier: &NSString,
            completion_handler: &Block<(*mut NSData, *mut NSError), ()>,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other loadFileRepresentationForTypeIdentifier:completionHandler:)]
        pub unsafe fn loadFileRepresentationForTypeIdentifier_completionHandler(
            &self,
            type_identifier: &NSString,
            completion_handler: &Block<(*mut NSURL, *mut NSError), ()>,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other loadInPlaceFileRepresentationForTypeIdentifier:completionHandler:)]
        pub unsafe fn loadInPlaceFileRepresentationForTypeIdentifier_completionHandler(
            &self,
            type_identifier: &NSString,
            completion_handler: &Block<(*mut NSURL, Bool, *mut NSError), ()>,
        ) -> Id<NSProgress>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other suggestedName)]
        pub unsafe fn suggestedName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSuggestedName:)]
        pub unsafe fn setSuggestedName(&self, suggested_name: Option<&NSString>);

        #[method_id(@__retain_semantics Init initWithObject:)]
        pub unsafe fn initWithObject(
            this: Option<Allocated<Self>>,
            object: &ProtocolObject<dyn NSItemProviderWriting>,
        ) -> Id<Self>;

        #[method(registerObject:visibility:)]
        pub unsafe fn registerObject_visibility(
            &self,
            object: &ProtocolObject<dyn NSItemProviderWriting>,
            visibility: NSItemProviderRepresentationVisibility,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithItem:typeIdentifier:)]
        pub unsafe fn initWithItem_typeIdentifier(
            this: Option<Allocated<Self>>,
            item: Option<&ProtocolObject<dyn NSSecureCoding>>,
            type_identifier: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            file_url: Option<&NSURL>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(registerItemForTypeIdentifier:loadHandler:)]
        pub unsafe fn registerItemForTypeIdentifier_loadHandler(
            &self,
            type_identifier: &NSString,
            load_handler: NSItemProviderLoadHandler,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(loadItemForTypeIdentifier:options:completionHandler:)]
        pub unsafe fn loadItemForTypeIdentifier_options_completionHandler(
            &self,
            type_identifier: &NSString,
            options: Option<&NSDictionary>,
            completion_handler: NSItemProviderCompletionHandler,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl NSItemProvider {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSItemProviderPreferredImageSizeKey: &'static NSString);

extern_methods!(
    /// NSPreviewSupport
    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl NSItemProvider {
        #[method(previewImageHandler)]
        pub unsafe fn previewImageHandler(&self) -> NSItemProviderLoadHandler;

        #[method(setPreviewImageHandler:)]
        pub unsafe fn setPreviewImageHandler(
            &self,
            preview_image_handler: NSItemProviderLoadHandler,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(loadPreviewImageWithOptions:completionHandler:)]
        pub unsafe fn loadPreviewImageWithOptions_completionHandler(
            &self,
            options: Option<&NSDictionary>,
            completion_handler: NSItemProviderCompletionHandler,
        );
    }
);

extern_static!(NSExtensionJavaScriptPreprocessingResultsKey: Option<&'static NSString>);

extern_static!(NSExtensionJavaScriptFinalizeArgumentKey: Option<&'static NSString>);

extern_static!(NSItemProviderErrorDomain: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSItemProviderErrorCode {
        NSItemProviderUnknownError = -1,
        NSItemProviderItemUnavailableError = -1000,
        NSItemProviderUnexpectedValueClassError = -1100,
        NSItemProviderUnavailableCoercionError = -1200,
    }
);