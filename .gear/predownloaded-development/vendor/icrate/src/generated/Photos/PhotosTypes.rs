//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHImageContentMode {
        PHImageContentModeAspectFit = 0,
        PHImageContentModeAspectFill = 1,
        PHImageContentModeDefault = PHImageContentModeAspectFit,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHCollectionListType {
        #[deprecated = "Will be removed in a future release"]
        PHCollectionListTypeMomentList = 1,
        PHCollectionListTypeFolder = 2,
        PHCollectionListTypeSmartFolder = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHCollectionListSubtype {
        #[deprecated = "Will be removed in a future release"]
        PHCollectionListSubtypeMomentListCluster = 1,
        #[deprecated = "Will be removed in a future release"]
        PHCollectionListSubtypeMomentListYear = 2,
        PHCollectionListSubtypeRegularFolder = 100,
        PHCollectionListSubtypeSmartFolderEvents = 200,
        PHCollectionListSubtypeSmartFolderFaces = 201,
        PHCollectionListSubtypeAny = NSIntegerMax as _,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHCollectionEditOperation {
        PHCollectionEditOperationDeleteContent = 1,
        PHCollectionEditOperationRemoveContent = 2,
        PHCollectionEditOperationAddContent = 3,
        PHCollectionEditOperationCreateContent = 4,
        PHCollectionEditOperationRearrangeContent = 5,
        PHCollectionEditOperationDelete = 6,
        PHCollectionEditOperationRename = 7,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHAssetCollectionType {
        PHAssetCollectionTypeAlbum = 1,
        PHAssetCollectionTypeSmartAlbum = 2,
        #[deprecated = "Will be removed in a future release"]
        PHAssetCollectionTypeMoment = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHAssetCollectionSubtype {
        PHAssetCollectionSubtypeAlbumRegular = 2,
        PHAssetCollectionSubtypeAlbumSyncedEvent = 3,
        PHAssetCollectionSubtypeAlbumSyncedFaces = 4,
        PHAssetCollectionSubtypeAlbumSyncedAlbum = 5,
        PHAssetCollectionSubtypeAlbumImported = 6,
        PHAssetCollectionSubtypeAlbumMyPhotoStream = 100,
        PHAssetCollectionSubtypeAlbumCloudShared = 101,
        PHAssetCollectionSubtypeSmartAlbumGeneric = 200,
        PHAssetCollectionSubtypeSmartAlbumPanoramas = 201,
        PHAssetCollectionSubtypeSmartAlbumVideos = 202,
        PHAssetCollectionSubtypeSmartAlbumFavorites = 203,
        PHAssetCollectionSubtypeSmartAlbumTimelapses = 204,
        PHAssetCollectionSubtypeSmartAlbumAllHidden = 205,
        PHAssetCollectionSubtypeSmartAlbumRecentlyAdded = 206,
        PHAssetCollectionSubtypeSmartAlbumBursts = 207,
        PHAssetCollectionSubtypeSmartAlbumSlomoVideos = 208,
        PHAssetCollectionSubtypeSmartAlbumUserLibrary = 209,
        PHAssetCollectionSubtypeSmartAlbumSelfPortraits = 210,
        PHAssetCollectionSubtypeSmartAlbumScreenshots = 211,
        PHAssetCollectionSubtypeSmartAlbumDepthEffect = 212,
        PHAssetCollectionSubtypeSmartAlbumLivePhotos = 213,
        PHAssetCollectionSubtypeSmartAlbumAnimated = 214,
        PHAssetCollectionSubtypeSmartAlbumLongExposures = 215,
        PHAssetCollectionSubtypeSmartAlbumUnableToUpload = 216,
        PHAssetCollectionSubtypeSmartAlbumRAW = 217,
        PHAssetCollectionSubtypeSmartAlbumCinematic = 218,
        PHAssetCollectionSubtypeAny = NSIntegerMax as _,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHAssetEditOperation {
        PHAssetEditOperationDelete = 1,
        PHAssetEditOperationContent = 2,
        PHAssetEditOperationProperties = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHAssetPlaybackStyle {
        PHAssetPlaybackStyleUnsupported = 0,
        PHAssetPlaybackStyleImage = 1,
        PHAssetPlaybackStyleImageAnimated = 2,
        PHAssetPlaybackStyleLivePhoto = 3,
        PHAssetPlaybackStyleVideo = 4,
        PHAssetPlaybackStyleVideoLooping = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHAssetMediaType {
        PHAssetMediaTypeUnknown = 0,
        PHAssetMediaTypeImage = 1,
        PHAssetMediaTypeVideo = 2,
        PHAssetMediaTypeAudio = 3,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum PHAssetMediaSubtype {
        PHAssetMediaSubtypeNone = 0,
        PHAssetMediaSubtypePhotoPanorama = 1 << 0,
        PHAssetMediaSubtypePhotoHDR = 1 << 1,
        PHAssetMediaSubtypePhotoScreenshot = 1 << 2,
        PHAssetMediaSubtypePhotoLive = 1 << 3,
        PHAssetMediaSubtypePhotoDepthEffect = 1 << 4,
        PHAssetMediaSubtypeVideoStreamed = 1 << 16,
        PHAssetMediaSubtypeVideoHighFrameRate = 1 << 17,
        PHAssetMediaSubtypeVideoTimelapse = 1 << 18,
        PHAssetMediaSubtypeVideoCinematic = 1 << 21,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum PHAssetBurstSelectionType {
        PHAssetBurstSelectionTypeNone = 0,
        PHAssetBurstSelectionTypeAutoPick = 1 << 0,
        PHAssetBurstSelectionTypeUserPick = 1 << 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum PHAssetSourceType {
        PHAssetSourceTypeNone = 0,
        PHAssetSourceTypeUserLibrary = 1 << 0,
        PHAssetSourceTypeCloudShared = 1 << 1,
        PHAssetSourceTypeiTunesSynced = 1 << 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHAssetResourceType {
        PHAssetResourceTypePhoto = 1,
        PHAssetResourceTypeVideo = 2,
        PHAssetResourceTypeAudio = 3,
        PHAssetResourceTypeAlternatePhoto = 4,
        PHAssetResourceTypeFullSizePhoto = 5,
        PHAssetResourceTypeFullSizeVideo = 6,
        PHAssetResourceTypeAdjustmentData = 7,
        PHAssetResourceTypeAdjustmentBasePhoto = 8,
        PHAssetResourceTypePairedVideo = 9,
        PHAssetResourceTypeFullSizePairedVideo = 10,
        PHAssetResourceTypeAdjustmentBasePairedVideo = 11,
        PHAssetResourceTypeAdjustmentBaseVideo = 12,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHObjectType {
        PHObjectTypeAsset = 1,
        PHObjectTypeAssetCollection = 2,
        PHObjectTypeCollectionList = 3,
    }
);
