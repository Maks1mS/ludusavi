//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_methods!(
    /// MPAdditions
    #[cfg(feature = "AVFoundation_AVPlayerItem")]
    unsafe impl AVPlayerItem {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other nowPlayingInfo)]
        pub unsafe fn nowPlayingInfo(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setNowPlayingInfo:)]
        pub unsafe fn setNowPlayingInfo(
            &self,
            now_playing_info: Option<&NSDictionary<NSString, AnyObject>>,
        );
    }
);
