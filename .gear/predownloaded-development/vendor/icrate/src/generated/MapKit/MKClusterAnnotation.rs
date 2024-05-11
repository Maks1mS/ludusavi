//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKClusterAnnotation")]
    pub struct MKClusterAnnotation;

    #[cfg(feature = "MapKit_MKClusterAnnotation")]
    unsafe impl ClassType for MKClusterAnnotation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKClusterAnnotation")]
unsafe impl MKAnnotation for MKClusterAnnotation {}

#[cfg(feature = "MapKit_MKClusterAnnotation")]
unsafe impl NSObjectProtocol for MKClusterAnnotation {}

extern_methods!(
    #[cfg(feature = "MapKit_MKClusterAnnotation")]
    unsafe impl MKClusterAnnotation {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other memberAnnotations)]
        pub unsafe fn memberAnnotations(&self) -> Id<NSArray<ProtocolObject<dyn MKAnnotation>>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithMemberAnnotations:)]
        pub unsafe fn initWithMemberAnnotations(
            this: Option<Allocated<Self>>,
            member_annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKClusterAnnotation")]
    unsafe impl MKClusterAnnotation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);