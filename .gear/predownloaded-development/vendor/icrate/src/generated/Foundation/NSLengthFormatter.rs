//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSLengthFormatterUnit {
        NSLengthFormatterUnitMillimeter = 8,
        NSLengthFormatterUnitCentimeter = 9,
        NSLengthFormatterUnitMeter = 11,
        NSLengthFormatterUnitKilometer = 14,
        NSLengthFormatterUnitInch = (5 << 8) + 1,
        NSLengthFormatterUnitFoot = (5 << 8) + 2,
        NSLengthFormatterUnitYard = (5 << 8) + 3,
        NSLengthFormatterUnitMile = (5 << 8) + 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSLengthFormatter")]
    pub struct NSLengthFormatter;

    #[cfg(feature = "Foundation_NSLengthFormatter")]
    unsafe impl ClassType for NSLengthFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSLengthFormatter")]
unsafe impl NSCoding for NSLengthFormatter {}

#[cfg(feature = "Foundation_NSLengthFormatter")]
unsafe impl NSCopying for NSLengthFormatter {}

#[cfg(feature = "Foundation_NSLengthFormatter")]
unsafe impl NSObjectProtocol for NSLengthFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSLengthFormatter")]
    unsafe impl NSLengthFormatter {
        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method_id(@__retain_semantics Other numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter>;

        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method(setNumberFormatter:)]
        pub unsafe fn setNumberFormatter(&self, number_formatter: Option<&NSNumberFormatter>);

        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;

        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unit_style: NSFormattingUnitStyle);

        #[method(isForPersonHeightUse)]
        pub unsafe fn isForPersonHeightUse(&self) -> bool;

        #[method(setForPersonHeightUse:)]
        pub unsafe fn setForPersonHeightUse(&self, for_person_height_use: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromValue:unit:)]
        pub unsafe fn stringFromValue_unit(
            &self,
            value: c_double,
            unit: NSLengthFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromMeters:)]
        pub unsafe fn stringFromMeters(&self, number_in_meters: c_double) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromValue:unit:)]
        pub unsafe fn unitStringFromValue_unit(
            &self,
            value: c_double,
            unit: NSLengthFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromMeters:usedUnit:)]
        pub unsafe fn unitStringFromMeters_usedUnit(
            &self,
            number_in_meters: c_double,
            unitp: *mut NSLengthFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<AnyObject>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSLengthFormatter")]
    unsafe impl NSLengthFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);