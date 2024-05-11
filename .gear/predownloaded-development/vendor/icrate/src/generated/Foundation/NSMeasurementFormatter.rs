//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMeasurementFormatterUnitOptions {
        NSMeasurementFormatterUnitOptionsProvidedUnit = 1 << 0,
        NSMeasurementFormatterUnitOptionsNaturalScale = 1 << 1,
        NSMeasurementFormatterUnitOptionsTemperatureWithoutUnit = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMeasurementFormatter")]
    pub struct NSMeasurementFormatter;

    #[cfg(feature = "Foundation_NSMeasurementFormatter")]
    unsafe impl ClassType for NSMeasurementFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSMeasurementFormatter")]
unsafe impl NSCoding for NSMeasurementFormatter {}

#[cfg(feature = "Foundation_NSMeasurementFormatter")]
unsafe impl NSCopying for NSMeasurementFormatter {}

#[cfg(feature = "Foundation_NSMeasurementFormatter")]
unsafe impl NSObjectProtocol for NSMeasurementFormatter {}

#[cfg(feature = "Foundation_NSMeasurementFormatter")]
unsafe impl NSSecureCoding for NSMeasurementFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMeasurementFormatter")]
    unsafe impl NSMeasurementFormatter {
        #[method(unitOptions)]
        pub unsafe fn unitOptions(&self) -> NSMeasurementFormatterUnitOptions;

        #[method(setUnitOptions:)]
        pub unsafe fn setUnitOptions(&self, unit_options: NSMeasurementFormatterUnitOptions);

        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;

        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unit_style: NSFormattingUnitStyle);

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method_id(@__retain_semantics Other numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter>;

        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method(setNumberFormatter:)]
        pub unsafe fn setNumberFormatter(&self, number_formatter: Option<&NSNumberFormatter>);

        #[cfg(all(feature = "Foundation_NSMeasurement", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromMeasurement:)]
        pub unsafe fn stringFromMeasurement(&self, measurement: &NSMeasurement) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUnit"))]
        #[method_id(@__retain_semantics Other stringFromUnit:)]
        pub unsafe fn stringFromUnit(&self, unit: &NSUnit) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMeasurementFormatter")]
    unsafe impl NSMeasurementFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);