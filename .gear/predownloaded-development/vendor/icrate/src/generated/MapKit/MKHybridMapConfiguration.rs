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
    #[cfg(feature = "MapKit_MKHybridMapConfiguration")]
    pub struct MKHybridMapConfiguration;

    #[cfg(feature = "MapKit_MKHybridMapConfiguration")]
    unsafe impl ClassType for MKHybridMapConfiguration {
        #[inherits(NSObject)]
        type Super = MKMapConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKHybridMapConfiguration")]
unsafe impl NSCoding for MKHybridMapConfiguration {}

#[cfg(feature = "MapKit_MKHybridMapConfiguration")]
unsafe impl NSCopying for MKHybridMapConfiguration {}

#[cfg(feature = "MapKit_MKHybridMapConfiguration")]
unsafe impl NSObjectProtocol for MKHybridMapConfiguration {}

#[cfg(feature = "MapKit_MKHybridMapConfiguration")]
unsafe impl NSSecureCoding for MKHybridMapConfiguration {}

extern_methods!(
    #[cfg(feature = "MapKit_MKHybridMapConfiguration")]
    unsafe impl MKHybridMapConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithElevationStyle:)]
        pub unsafe fn initWithElevationStyle(
            this: Option<Allocated<Self>>,
            elevation_style: MKMapElevationStyle,
        ) -> Id<Self>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[method(showsTraffic)]
        pub unsafe fn showsTraffic(&self) -> bool;

        #[method(setShowsTraffic:)]
        pub unsafe fn setShowsTraffic(&self, shows_traffic: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKMapConfiguration`
    #[cfg(feature = "MapKit_MKHybridMapConfiguration")]
    unsafe impl MKHybridMapConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);