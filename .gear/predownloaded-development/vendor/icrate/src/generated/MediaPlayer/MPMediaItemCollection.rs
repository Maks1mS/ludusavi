//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
    pub struct MPMediaItemCollection;

    #[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
    unsafe impl ClassType for MPMediaItemCollection {
        #[inherits(NSObject)]
        type Super = MPMediaEntity;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
unsafe impl NSCoding for MPMediaItemCollection {}

#[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
unsafe impl NSObjectProtocol for MPMediaItemCollection {}

#[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
unsafe impl NSSecureCoding for MPMediaItemCollection {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
    unsafe impl MPMediaItemCollection {
        #[cfg(all(feature = "Foundation_NSArray", feature = "MediaPlayer_MPMediaItem"))]
        #[method_id(@__retain_semantics Other collectionWithItems:)]
        pub unsafe fn collectionWithItems(
            items: &NSArray<MPMediaItem>,
        ) -> Id<MPMediaItemCollection>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MediaPlayer_MPMediaItem"))]
        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Option<Allocated<Self>>,
            items: &NSArray<MPMediaItem>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MediaPlayer_MPMediaItem"))]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Id<NSArray<MPMediaItem>>;

        #[cfg(feature = "MediaPlayer_MPMediaItem")]
        #[method_id(@__retain_semantics Other representativeItem)]
        pub unsafe fn representativeItem(&self) -> Option<Id<MPMediaItem>>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method(mediaTypes)]
        pub unsafe fn mediaTypes(&self) -> MPMediaType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
    unsafe impl MPMediaItemCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);