//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::EventKit::*;
use crate::Foundation::*;
use crate::MapKit::*;

pub type EKVirtualConferenceRoomTypeIdentifier = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKVirtualConferenceRoomTypeDescriptor")]
    pub struct EKVirtualConferenceRoomTypeDescriptor;

    #[cfg(feature = "EventKit_EKVirtualConferenceRoomTypeDescriptor")]
    unsafe impl ClassType for EKVirtualConferenceRoomTypeDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "EventKit_EKVirtualConferenceRoomTypeDescriptor")]
unsafe impl NSObjectProtocol for EKVirtualConferenceRoomTypeDescriptor {}

extern_methods!(
    #[cfg(feature = "EventKit_EKVirtualConferenceRoomTypeDescriptor")]
    unsafe impl EKVirtualConferenceRoomTypeDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithTitle:identifier:)]
        pub unsafe fn initWithTitle_identifier(
            this: Option<Allocated<Self>>,
            title: &NSString,
            identifier: &EKVirtualConferenceRoomTypeIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<EKVirtualConferenceRoomTypeIdentifier>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKVirtualConferenceURLDescriptor")]
    pub struct EKVirtualConferenceURLDescriptor;

    #[cfg(feature = "EventKit_EKVirtualConferenceURLDescriptor")]
    unsafe impl ClassType for EKVirtualConferenceURLDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "EventKit_EKVirtualConferenceURLDescriptor")]
unsafe impl NSObjectProtocol for EKVirtualConferenceURLDescriptor {}

extern_methods!(
    #[cfg(feature = "EventKit_EKVirtualConferenceURLDescriptor")]
    unsafe impl EKVirtualConferenceURLDescriptor {
        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithTitle:URL:)]
        pub unsafe fn initWithTitle_URL(
            this: Option<Allocated<Self>>,
            title: Option<&NSString>,
            url: &NSURL,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKVirtualConferenceDescriptor")]
    pub struct EKVirtualConferenceDescriptor;

    #[cfg(feature = "EventKit_EKVirtualConferenceDescriptor")]
    unsafe impl ClassType for EKVirtualConferenceDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "EventKit_EKVirtualConferenceDescriptor")]
unsafe impl NSObjectProtocol for EKVirtualConferenceDescriptor {}

extern_methods!(
    #[cfg(feature = "EventKit_EKVirtualConferenceDescriptor")]
    unsafe impl EKVirtualConferenceDescriptor {
        #[cfg(all(
            feature = "EventKit_EKVirtualConferenceURLDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithTitle:URLDescriptors:conferenceDetails:)]
        pub unsafe fn initWithTitle_URLDescriptors_conferenceDetails(
            this: Option<Allocated<Self>>,
            title: Option<&NSString>,
            url_descriptors: &NSArray<EKVirtualConferenceURLDescriptor>,
            conference_details: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "EventKit_EKVirtualConferenceURLDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other URLDescriptors)]
        pub unsafe fn URLDescriptors(&self) -> Id<NSArray<EKVirtualConferenceURLDescriptor>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other conferenceDetails)]
        pub unsafe fn conferenceDetails(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);