//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_static!(HKErrorDomain: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKErrorCode {
        HKNoError = 0,
        HKErrorHealthDataUnavailable = 1,
        HKErrorHealthDataRestricted = 2,
        HKErrorInvalidArgument = 3,
        HKErrorAuthorizationDenied = 4,
        HKErrorAuthorizationNotDetermined = 5,
        HKErrorDatabaseInaccessible = 6,
        HKErrorUserCanceled = 7,
        HKErrorAnotherWorkoutSessionStarted = 8,
        HKErrorUserExitedWorkoutSession = 9,
        HKErrorRequiredAuthorizationDenied = 10,
        HKErrorNoData = 11,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKUpdateFrequency {
        HKUpdateFrequencyImmediate = 1,
        HKUpdateFrequencyHourly = 2,
        HKUpdateFrequencyDaily = 3,
        HKUpdateFrequencyWeekly = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKAuthorizationStatus {
        HKAuthorizationStatusNotDetermined = 0,
        HKAuthorizationStatusSharingDenied = 1,
        HKAuthorizationStatusSharingAuthorized = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKAuthorizationRequestStatus {
        HKAuthorizationRequestStatusUnknown = 0,
        HKAuthorizationRequestStatusShouldRequest = 1,
        HKAuthorizationRequestStatusUnnecessary = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKBiologicalSex {
        HKBiologicalSexNotSet = 0,
        HKBiologicalSexFemale = 1,
        HKBiologicalSexMale = 2,
        HKBiologicalSexOther = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKBloodType {
        HKBloodTypeNotSet = 0,
        HKBloodTypeAPositive = 1,
        HKBloodTypeANegative = 2,
        HKBloodTypeBPositive = 3,
        HKBloodTypeBNegative = 4,
        HKBloodTypeABPositive = 5,
        HKBloodTypeABNegative = 6,
        HKBloodTypeOPositive = 7,
        HKBloodTypeONegative = 8,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueSleepAnalysis {
        HKCategoryValueSleepAnalysisInBed = 0,
        HKCategoryValueSleepAnalysisAsleepUnspecified = 1,
        #[deprecated]
        HKCategoryValueSleepAnalysisAsleep = HKCategoryValueSleepAnalysisAsleepUnspecified,
        HKCategoryValueSleepAnalysisAwake = 2,
        HKCategoryValueSleepAnalysisAsleepCore = 3,
        HKCategoryValueSleepAnalysisAsleepDeep = 4,
        HKCategoryValueSleepAnalysisAsleepREM = 5,
    }
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSNumber", feature = "Foundation_NSSet"))]
    pub unsafe fn HKCategoryValueSleepAnalysisAsleepValues() -> NonNull<NSSet<NSNumber>>;
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueAppleStandHour {
        HKCategoryValueAppleStandHourStood = 0,
        HKCategoryValueAppleStandHourIdle = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKFitzpatrickSkinType {
        HKFitzpatrickSkinTypeNotSet = 0,
        HKFitzpatrickSkinTypeI = 1,
        HKFitzpatrickSkinTypeII = 2,
        HKFitzpatrickSkinTypeIII = 3,
        HKFitzpatrickSkinTypeIV = 4,
        HKFitzpatrickSkinTypeV = 5,
        HKFitzpatrickSkinTypeVI = 6,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKWheelchairUse {
        HKWheelchairUseNotSet = 0,
        HKWheelchairUseNo = 1,
        HKWheelchairUseYes = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueCervicalMucusQuality {
        HKCategoryValueCervicalMucusQualityDry = 1,
        HKCategoryValueCervicalMucusQualitySticky = 2,
        HKCategoryValueCervicalMucusQualityCreamy = 3,
        HKCategoryValueCervicalMucusQualityWatery = 4,
        HKCategoryValueCervicalMucusQualityEggWhite = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueOvulationTestResult {
        HKCategoryValueOvulationTestResultNegative = 1,
        HKCategoryValueOvulationTestResultLuteinizingHormoneSurge = 2,
        #[deprecated]
        HKCategoryValueOvulationTestResultPositive =
            HKCategoryValueOvulationTestResultLuteinizingHormoneSurge,
        HKCategoryValueOvulationTestResultIndeterminate = 3,
        HKCategoryValueOvulationTestResultEstrogenSurge = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValuePregnancyTestResult {
        HKCategoryValuePregnancyTestResultNegative = 1,
        HKCategoryValuePregnancyTestResultPositive = 2,
        HKCategoryValuePregnancyTestResultIndeterminate = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueProgesteroneTestResult {
        HKCategoryValueProgesteroneTestResultNegative = 1,
        HKCategoryValueProgesteroneTestResultPositive = 2,
        HKCategoryValueProgesteroneTestResultIndeterminate = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueMenstrualFlow {
        HKCategoryValueMenstrualFlowUnspecified = 1,
        HKCategoryValueMenstrualFlowLight = 2,
        HKCategoryValueMenstrualFlowMedium = 3,
        HKCategoryValueMenstrualFlowHeavy = 4,
        HKCategoryValueMenstrualFlowNone = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValue {
        HKCategoryValueNotApplicable = 0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated]
    pub enum HKCategoryValueAudioExposureEvent {
        #[deprecated]
        HKCategoryValueAudioExposureEventLoudEnvironment = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueEnvironmentalAudioExposureEvent {
        HKCategoryValueEnvironmentalAudioExposureEventMomentaryLimit = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueContraceptive {
        HKCategoryValueContraceptiveUnspecified = 1,
        HKCategoryValueContraceptiveImplant = 2,
        HKCategoryValueContraceptiveInjection = 3,
        HKCategoryValueContraceptiveIntrauterineDevice = 4,
        HKCategoryValueContraceptiveIntravaginalRing = 5,
        HKCategoryValueContraceptiveOral = 6,
        HKCategoryValueContraceptivePatch = 7,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueSeverity {
        HKCategoryValueSeverityUnspecified = 0,
        HKCategoryValueSeverityNotPresent = 1,
        HKCategoryValueSeverityMild = 2,
        HKCategoryValueSeverityModerate = 3,
        HKCategoryValueSeveritySevere = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueAppetiteChanges {
        HKCategoryValueAppetiteChangesUnspecified = 0,
        HKCategoryValueAppetiteChangesNoChange = 1,
        HKCategoryValueAppetiteChangesDecreased = 2,
        HKCategoryValueAppetiteChangesIncreased = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValuePresence {
        HKCategoryValuePresencePresent = 0,
        HKCategoryValuePresenceNotPresent = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueHeadphoneAudioExposureEvent {
        HKCategoryValueHeadphoneAudioExposureEventSevenDayLimit = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueLowCardioFitnessEvent {
        HKCategoryValueLowCardioFitnessEventLowFitness = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKActivityMoveMode {
        HKActivityMoveModeActiveEnergy = 1,
        HKActivityMoveModeAppleMoveTime = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKCategoryValueAppleWalkingSteadinessEvent {
        HKCategoryValueAppleWalkingSteadinessEventInitialLow = 1,
        HKCategoryValueAppleWalkingSteadinessEventInitialVeryLow = 2,
        HKCategoryValueAppleWalkingSteadinessEventRepeatLow = 3,
        HKCategoryValueAppleWalkingSteadinessEventRepeatVeryLow = 4,
    }
);
