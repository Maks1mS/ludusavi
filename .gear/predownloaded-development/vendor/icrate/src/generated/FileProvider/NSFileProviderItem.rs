//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

typed_extensible_enum!(
    pub type NSFileProviderItemIdentifier = NSString;
);

extern_static!(NSFileProviderRootContainerItemIdentifier: &'static NSFileProviderItemIdentifier);

extern_static!(
    NSFileProviderWorkingSetContainerItemIdentifier: &'static NSFileProviderItemIdentifier
);

extern_static!(NSFileProviderTrashContainerItemIdentifier: &'static NSFileProviderItemIdentifier);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
    pub struct NSFileProviderItemVersion;

    #[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
    unsafe impl ClassType for NSFileProviderItemVersion {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
unsafe impl NSObjectProtocol for NSFileProviderItemVersion {}

extern_methods!(
    #[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
    unsafe impl NSFileProviderItemVersion {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other beforeFirstSyncComponent)]
        pub unsafe fn beforeFirstSyncComponent() -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithContentVersion:metadataVersion:)]
        pub unsafe fn initWithContentVersion_metadataVersion(
            this: Option<Allocated<Self>>,
            content_version: &NSData,
            metadata_version: &NSData,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other contentVersion)]
        pub unsafe fn contentVersion(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other metadataVersion)]
        pub unsafe fn metadataVersion(&self) -> Id<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
    unsafe impl NSFileProviderItemVersion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSFileProviderFavoriteRankUnranked: c_ulonglong);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileProviderItemCapabilities {
        NSFileProviderItemCapabilitiesAllowsReading = 1 << 0,
        NSFileProviderItemCapabilitiesAllowsWriting = 1 << 1,
        NSFileProviderItemCapabilitiesAllowsReparenting = 1 << 2,
        NSFileProviderItemCapabilitiesAllowsRenaming = 1 << 3,
        NSFileProviderItemCapabilitiesAllowsTrashing = 1 << 4,
        NSFileProviderItemCapabilitiesAllowsDeleting = 1 << 5,
        #[deprecated = "use NSFileProviderContentPolicy instead"]
        NSFileProviderItemCapabilitiesAllowsEvicting = 1 << 6,
        NSFileProviderItemCapabilitiesAllowsExcludingFromSync = 1 << 7,
        NSFileProviderItemCapabilitiesAllowsAddingSubItems =
            NSFileProviderItemCapabilitiesAllowsWriting,
        NSFileProviderItemCapabilitiesAllowsContentEnumerating =
            NSFileProviderItemCapabilitiesAllowsReading,
        #[deprecated = "This capability is no longer supported, and does not contain all capabilities. Please migrate to directly specifying each of the individual capabilities that should be allowed for the item."]
        NSFileProviderItemCapabilitiesAllowsAll = NSFileProviderItemCapabilitiesAllowsReading
            | NSFileProviderItemCapabilitiesAllowsWriting
            | NSFileProviderItemCapabilitiesAllowsReparenting
            | NSFileProviderItemCapabilitiesAllowsRenaming
            | NSFileProviderItemCapabilitiesAllowsTrashing
            | NSFileProviderItemCapabilitiesAllowsDeleting,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileProviderItemFields {
        NSFileProviderItemContents = 1 << 0,
        NSFileProviderItemFilename = 1 << 1,
        NSFileProviderItemParentItemIdentifier = 1 << 2,
        NSFileProviderItemLastUsedDate = 1 << 3,
        NSFileProviderItemTagData = 1 << 4,
        NSFileProviderItemFavoriteRank = 1 << 5,
        NSFileProviderItemCreationDate = 1 << 6,
        NSFileProviderItemContentModificationDate = 1 << 7,
        NSFileProviderItemFileSystemFlags = 1 << 8,
        NSFileProviderItemExtendedAttributes = 1 << 9,
        NSFileProviderItemTypeAndCreator = 1 << 10,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileProviderFileSystemFlags {
        NSFileProviderFileSystemUserExecutable = 1 << 0,
        NSFileProviderFileSystemUserReadable = 1 << 1,
        NSFileProviderFileSystemUserWritable = 1 << 2,
        NSFileProviderFileSystemHidden = 1 << 3,
        NSFileProviderFileSystemPathExtensionHidden = 1 << 4,
    }
);

extern_struct!(
    pub struct NSFileProviderTypeAndCreator {
        pub r#type: OSType,
        pub creator: OSType,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSFileProviderContentPolicy {
        NSFileProviderContentPolicyInherited = 0,
        NSFileProviderContentPolicyDownloadLazily = 1,
        NSFileProviderContentPolicyDownloadLazilyAndEvictOnRemoteUpdate = 2,
        NSFileProviderContentPolicyDownloadEagerlyAndKeepDownloaded = 3,
    }
);

extern_protocol!(
    pub unsafe trait NSFileProviderItemProtocol: NSObjectProtocol {
        #[method_id(@__retain_semantics Other itemIdentifier)]
        unsafe fn itemIdentifier(&self) -> Id<NSFileProviderItemIdentifier>;

        #[method_id(@__retain_semantics Other parentItemIdentifier)]
        unsafe fn parentItemIdentifier(&self) -> Id<NSFileProviderItemIdentifier>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other filename)]
        unsafe fn filename(&self) -> Id<NSString>;

        #[cfg(feature = "UniformTypeIdentifiers_UTType")]
        #[optional]
        #[method_id(@__retain_semantics Other contentType)]
        unsafe fn contentType(&self) -> Id<UTType>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other typeIdentifier)]
        unsafe fn typeIdentifier(&self) -> Id<NSString>;

        #[optional]
        #[method(typeAndCreator)]
        unsafe fn typeAndCreator(&self) -> NSFileProviderTypeAndCreator;

        #[optional]
        #[method(capabilities)]
        unsafe fn capabilities(&self) -> NSFileProviderItemCapabilities;

        #[optional]
        #[method(fileSystemFlags)]
        unsafe fn fileSystemFlags(&self) -> NSFileProviderFileSystemFlags;

        #[cfg(feature = "Foundation_NSNumber")]
        #[optional]
        #[method_id(@__retain_semantics Other documentSize)]
        unsafe fn documentSize(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[optional]
        #[method_id(@__retain_semantics Other childItemCount)]
        unsafe fn childItemCount(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[optional]
        #[method_id(@__retain_semantics Other creationDate)]
        unsafe fn creationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[optional]
        #[method_id(@__retain_semantics Other contentModificationDate)]
        unsafe fn contentModificationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other extendedAttributes)]
        unsafe fn extendedAttributes(&self) -> Id<NSDictionary<NSString, NSData>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[optional]
        #[method_id(@__retain_semantics Other lastUsedDate)]
        unsafe fn lastUsedDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSData")]
        #[optional]
        #[method_id(@__retain_semantics Other tagData)]
        unsafe fn tagData(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[optional]
        #[method_id(@__retain_semantics Other favoriteRank)]
        unsafe fn favoriteRank(&self) -> Option<Id<NSNumber>>;

        #[optional]
        #[method(isTrashed)]
        unsafe fn isTrashed(&self) -> bool;

        #[optional]
        #[method(isUploaded)]
        unsafe fn isUploaded(&self) -> bool;

        #[optional]
        #[method(isUploading)]
        unsafe fn isUploading(&self) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[optional]
        #[method_id(@__retain_semantics Other uploadingError)]
        unsafe fn uploadingError(&self) -> Option<Id<NSError>>;

        #[optional]
        #[method(isDownloaded)]
        unsafe fn isDownloaded(&self) -> bool;

        #[optional]
        #[method(isDownloading)]
        unsafe fn isDownloading(&self) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[optional]
        #[method_id(@__retain_semantics Other downloadingError)]
        unsafe fn downloadingError(&self) -> Option<Id<NSError>>;

        #[optional]
        #[method(isMostRecentVersionDownloaded)]
        unsafe fn isMostRecentVersionDownloaded(&self) -> bool;

        #[optional]
        #[method(isShared)]
        unsafe fn isShared(&self) -> bool;

        #[optional]
        #[method(isSharedByCurrentUser)]
        unsafe fn isSharedByCurrentUser(&self) -> bool;

        #[cfg(feature = "Foundation_NSPersonNameComponents")]
        #[optional]
        #[method_id(@__retain_semantics Other ownerNameComponents)]
        unsafe fn ownerNameComponents(&self) -> Option<Id<NSPersonNameComponents>>;

        #[cfg(feature = "Foundation_NSPersonNameComponents")]
        #[optional]
        #[method_id(@__retain_semantics Other mostRecentEditorNameComponents)]
        unsafe fn mostRecentEditorNameComponents(&self) -> Option<Id<NSPersonNameComponents>>;

        #[cfg(feature = "Foundation_NSData")]
        #[optional]
        #[method_id(@__retain_semantics Other versionIdentifier)]
        unsafe fn versionIdentifier(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
        #[optional]
        #[method_id(@__retain_semantics Other itemVersion)]
        unsafe fn itemVersion(&self) -> Id<NSFileProviderItemVersion>;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method_id(@__retain_semantics Other symlinkTargetPath)]
        unsafe fn symlinkTargetPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[optional]
        #[method_id(@__retain_semantics Other userInfo)]
        unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

        #[optional]
        #[method(contentPolicy)]
        unsafe fn contentPolicy(&self) -> NSFileProviderContentPolicy;
    }

    unsafe impl ProtocolType for dyn NSFileProviderItemProtocol {}
);

pub type NSFileProviderItem = ProtocolObject<dyn NSFileProviderItemProtocol>;
