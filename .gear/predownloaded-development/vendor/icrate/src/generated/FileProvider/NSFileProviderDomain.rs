//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

typed_extensible_enum!(
    pub type NSFileProviderDomainIdentifier = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
    pub struct NSFileProviderDomainVersion;

    #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
    unsafe impl ClassType for NSFileProviderDomainVersion {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
unsafe impl NSCoding for NSFileProviderDomainVersion {}

#[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
unsafe impl NSObjectProtocol for NSFileProviderDomainVersion {}

#[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
unsafe impl NSSecureCoding for NSFileProviderDomainVersion {}

extern_methods!(
    #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
    unsafe impl NSFileProviderDomainVersion {
        #[method_id(@__retain_semantics Other next)]
        pub unsafe fn next(&self) -> Id<NSFileProviderDomainVersion>;

        #[method(compare:)]
        pub unsafe fn compare(
            &self,
            other_version: &NSFileProviderDomainVersion,
        ) -> NSComparisonResult;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
    unsafe impl NSFileProviderDomainVersion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileProviderDomainTestingModes {
        NSFileProviderDomainTestingModeAlwaysEnabled = 1 << 0,
        NSFileProviderDomainTestingModeInteractive = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "FileProvider_NSFileProviderDomain")]
    pub struct NSFileProviderDomain;

    #[cfg(feature = "FileProvider_NSFileProviderDomain")]
    unsafe impl ClassType for NSFileProviderDomain {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "FileProvider_NSFileProviderDomain")]
unsafe impl NSObjectProtocol for NSFileProviderDomain {}

extern_methods!(
    #[cfg(feature = "FileProvider_NSFileProviderDomain")]
    unsafe impl NSFileProviderDomain {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:displayName:pathRelativeToDocumentStorage:)]
        pub unsafe fn initWithIdentifier_displayName_pathRelativeToDocumentStorage(
            this: Option<Allocated<Self>>,
            identifier: &NSFileProviderDomainIdentifier,
            display_name: &NSString,
            path_relative_to_document_storage: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:displayName:)]
        pub unsafe fn initWithIdentifier_displayName(
            this: Option<Allocated<Self>>,
            identifier: &NSFileProviderDomainIdentifier,
            display_name: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSFileProviderDomainIdentifier>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathRelativeToDocumentStorage)]
        pub unsafe fn pathRelativeToDocumentStorage(&self) -> Id<NSString>;

        #[method(isDisconnected)]
        pub unsafe fn isDisconnected(&self) -> bool;

        #[method(userEnabled)]
        pub unsafe fn userEnabled(&self) -> bool;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method(isReplicated)]
        pub unsafe fn isReplicated(&self) -> bool;

        #[method(testingModes)]
        pub unsafe fn testingModes(&self) -> NSFileProviderDomainTestingModes;

        #[method(setTestingModes:)]
        pub unsafe fn setTestingModes(&self, testing_modes: NSFileProviderDomainTestingModes);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other backingStoreIdentity)]
        pub unsafe fn backingStoreIdentity(&self) -> Option<Id<NSData>>;

        #[method(supportsSyncingTrash)]
        pub unsafe fn supportsSyncingTrash(&self) -> bool;

        #[method(setSupportsSyncingTrash:)]
        pub unsafe fn setSupportsSyncingTrash(&self, supports_syncing_trash: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "FileProvider_NSFileProviderDomain")]
    unsafe impl NSFileProviderDomain {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSFileProviderDomain
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(feature = "FileProvider_NSFileProviderDomain")]
        #[method_id(@__retain_semantics Other domain)]
        pub unsafe fn domain(&self) -> Option<Id<NSFileProviderDomain>>;
    }
);

extern_static!(NSFileProviderDomainDidChange: &'static NSNotificationName);
