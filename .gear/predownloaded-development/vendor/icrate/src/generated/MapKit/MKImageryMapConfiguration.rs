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
    #[cfg(feature = "MapKit_MKImageryMapConfiguration")]
    pub struct MKImageryMapConfiguration;

    #[cfg(feature = "MapKit_MKImageryMapConfiguration")]
    unsafe impl ClassType for MKImageryMapConfiguration {
        #[inherits(NSObject)]
        type Super = MKMapConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKImageryMapConfiguration")]
unsafe impl NSCoding for MKImageryMapConfiguration {}

#[cfg(feature = "MapKit_MKImageryMapConfiguration")]
unsafe impl NSCopying for MKImageryMapConfiguration {}

#[cfg(feature = "MapKit_MKImageryMapConfiguration")]
unsafe impl NSObjectProtocol for MKImageryMapConfiguration {}

#[cfg(feature = "MapKit_MKImageryMapConfiguration")]
unsafe impl NSSecureCoding for MKImageryMapConfiguration {}

extern_methods!(
    #[cfg(feature = "MapKit_MKImageryMapConfiguration")]
    unsafe impl MKImageryMapConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithElevationStyle:)]
        pub unsafe fn initWithElevationStyle(
            this: Option<Allocated<Self>>,
            elevation_style: MKMapElevationStyle,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MKMapConfiguration`
    #[cfg(feature = "MapKit_MKImageryMapConfiguration")]
    unsafe impl MKImageryMapConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
