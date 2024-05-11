//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

extern_static!(NSFileProviderErrorDomain: &'static NSErrorDomain);

extern_static!(NSFileProviderErrorCollidingItemKey: &'static NSErrorUserInfoKey);

extern_static!(NSFileProviderErrorItemKey: &'static NSErrorUserInfoKey);

extern_static!(NSFileProviderErrorNonExistentItemIdentifierKey: &'static NSErrorUserInfoKey);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum NSFileProviderErrorCode {
        NSFileProviderErrorNotAuthenticated = -1000,
        NSFileProviderErrorFilenameCollision = -1001,
        NSFileProviderErrorSyncAnchorExpired = -1002,
        NSFileProviderErrorPageExpired = NSFileProviderErrorSyncAnchorExpired,
        NSFileProviderErrorInsufficientQuota = -1003,
        NSFileProviderErrorServerUnreachable = -1004,
        NSFileProviderErrorNoSuchItem = -1005,
        NSFileProviderErrorDeletionRejected = -1006,
        NSFileProviderErrorDirectoryNotEmpty = -1007,
        NSFileProviderErrorProviderNotFound = -2001,
        NSFileProviderErrorProviderTranslocated = -2002,
        NSFileProviderErrorOlderExtensionVersionRunning = -2003,
        NSFileProviderErrorNewerExtensionVersionFound = -2004,
        NSFileProviderErrorCannotSynchronize = -2005,
        NSFileProviderErrorNonEvictableChildren = -2006,
        NSFileProviderErrorUnsyncedEdits = -2007,
        NSFileProviderErrorNonEvictable = -2008,
        NSFileProviderErrorVersionNoLongerAvailable = -2009,
        NSFileProviderErrorExcludedFromSync = -2010,
        NSFileProviderErrorDomainDisabled = -2011,
    }
);

extern_methods!(
    /// NSFileProviderError
    #[cfg(feature = "Foundation_NSError")]
    unsafe impl NSError {
        #[method_id(@__retain_semantics Other fileProviderErrorForCollisionWithItem:)]
        pub unsafe fn fileProviderErrorForCollisionWithItem(
            existing_item: &NSFileProviderItem,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other fileProviderErrorForNonExistentItemWithIdentifier:)]
        pub unsafe fn fileProviderErrorForNonExistentItemWithIdentifier(
            item_identifier: &NSFileProviderItemIdentifier,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other fileProviderErrorForRejectedDeletionOfItem:)]
        pub unsafe fn fileProviderErrorForRejectedDeletionOfItem(
            updated_version: &NSFileProviderItem,
        ) -> Id<Self>;
    }
);