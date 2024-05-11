//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFTranscription")]
    pub struct SFTranscription;

    #[cfg(feature = "Speech_SFTranscription")]
    unsafe impl ClassType for SFTranscription {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech_SFTranscription")]
unsafe impl NSCoding for SFTranscription {}

#[cfg(feature = "Speech_SFTranscription")]
unsafe impl NSCopying for SFTranscription {}

#[cfg(feature = "Speech_SFTranscription")]
unsafe impl NSObjectProtocol for SFTranscription {}

#[cfg(feature = "Speech_SFTranscription")]
unsafe impl NSSecureCoding for SFTranscription {}

extern_methods!(
    #[cfg(feature = "Speech_SFTranscription")]
    unsafe impl SFTranscription {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other formattedString)]
        pub unsafe fn formattedString(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Speech_SFTranscriptionSegment"
        ))]
        #[method_id(@__retain_semantics Other segments)]
        pub unsafe fn segments(&self) -> Id<NSArray<SFTranscriptionSegment>>;

        #[deprecated = "speakingRate is moved to SFSpeechRecognitionMetadata"]
        #[method(speakingRate)]
        pub unsafe fn speakingRate(&self) -> c_double;

        #[deprecated = "averagePauseDuration is moved to SFSpeechRecognitionMetadata"]
        #[method(averagePauseDuration)]
        pub unsafe fn averagePauseDuration(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech_SFTranscription")]
    unsafe impl SFTranscription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);