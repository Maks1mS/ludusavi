//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKWorkoutSessionLocationType {
        HKWorkoutSessionLocationTypeUnknown = 1,
        HKWorkoutSessionLocationTypeIndoor = 2,
        HKWorkoutSessionLocationTypeOutdoor = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
    pub struct HKWorkoutConfiguration;

    #[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
    unsafe impl ClassType for HKWorkoutConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
unsafe impl NSCoding for HKWorkoutConfiguration {}

#[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
unsafe impl NSCopying for HKWorkoutConfiguration {}

#[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
unsafe impl NSObjectProtocol for HKWorkoutConfiguration {}

#[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
unsafe impl NSSecureCoding for HKWorkoutConfiguration {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
    unsafe impl HKWorkoutConfiguration {
        #[method(activityType)]
        pub unsafe fn activityType(&self) -> HKWorkoutActivityType;

        #[method(setActivityType:)]
        pub unsafe fn setActivityType(&self, activity_type: HKWorkoutActivityType);

        #[method(locationType)]
        pub unsafe fn locationType(&self) -> HKWorkoutSessionLocationType;

        #[method(setLocationType:)]
        pub unsafe fn setLocationType(&self, location_type: HKWorkoutSessionLocationType);

        #[method(swimmingLocationType)]
        pub unsafe fn swimmingLocationType(&self) -> HKWorkoutSwimmingLocationType;

        #[method(setSwimmingLocationType:)]
        pub unsafe fn setSwimmingLocationType(
            &self,
            swimming_location_type: HKWorkoutSwimmingLocationType,
        );

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other lapLength)]
        pub unsafe fn lapLength(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method(setLapLength:)]
        pub unsafe fn setLapLength(&self, lap_length: Option<&HKQuantity>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
    unsafe impl HKWorkoutConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);