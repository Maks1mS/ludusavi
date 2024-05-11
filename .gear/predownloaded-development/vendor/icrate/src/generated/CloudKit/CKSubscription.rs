//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKSubscriptionType {
        CKSubscriptionTypeQuery = 1,
        CKSubscriptionTypeRecordZone = 2,
        CKSubscriptionTypeDatabase = 3,
    }
);

pub type CKSubscriptionID = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSubscription")]
    pub struct CKSubscription;

    #[cfg(feature = "CloudKit_CKSubscription")]
    unsafe impl ClassType for CKSubscription {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKSubscription")]
unsafe impl NSCoding for CKSubscription {}

#[cfg(feature = "CloudKit_CKSubscription")]
unsafe impl NSCopying for CKSubscription {}

#[cfg(feature = "CloudKit_CKSubscription")]
unsafe impl NSObjectProtocol for CKSubscription {}

#[cfg(feature = "CloudKit_CKSubscription")]
unsafe impl NSSecureCoding for CKSubscription {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKSubscription")]
    unsafe impl CKSubscription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Other subscriptionID)]
        pub unsafe fn subscriptionID(&self) -> Id<CKSubscriptionID>;

        #[method(subscriptionType)]
        pub unsafe fn subscriptionType(&self) -> CKSubscriptionType;

        #[cfg(feature = "CloudKit_CKNotificationInfo")]
        #[method_id(@__retain_semantics Other notificationInfo)]
        pub unsafe fn notificationInfo(&self) -> Option<Id<CKNotificationInfo>>;

        #[cfg(feature = "CloudKit_CKNotificationInfo")]
        #[method(setNotificationInfo:)]
        pub unsafe fn setNotificationInfo(&self, notification_info: Option<&CKNotificationInfo>);
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum CKQuerySubscriptionOptions {
        CKQuerySubscriptionOptionsFiresOnRecordCreation = 1 << 0,
        CKQuerySubscriptionOptionsFiresOnRecordUpdate = 1 << 1,
        CKQuerySubscriptionOptionsFiresOnRecordDeletion = 1 << 2,
        CKQuerySubscriptionOptionsFiresOnce = 1 << 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKQuerySubscription")]
    pub struct CKQuerySubscription;

    #[cfg(feature = "CloudKit_CKQuerySubscription")]
    unsafe impl ClassType for CKQuerySubscription {
        #[inherits(NSObject)]
        type Super = CKSubscription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKQuerySubscription")]
unsafe impl NSCoding for CKQuerySubscription {}

#[cfg(feature = "CloudKit_CKQuerySubscription")]
unsafe impl NSCopying for CKQuerySubscription {}

#[cfg(feature = "CloudKit_CKQuerySubscription")]
unsafe impl NSObjectProtocol for CKQuerySubscription {}

#[cfg(feature = "CloudKit_CKQuerySubscription")]
unsafe impl NSSecureCoding for CKQuerySubscription {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKQuerySubscription")]
    unsafe impl CKQuerySubscription {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Init initWithRecordType:predicate:options:)]
        pub unsafe fn initWithRecordType_predicate_options(
            this: Option<Allocated<Self>>,
            record_type: &CKRecordType,
            predicate: &NSPredicate,
            query_subscription_options: CKQuerySubscriptionOptions,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Init initWithRecordType:predicate:subscriptionID:options:)]
        pub unsafe fn initWithRecordType_predicate_subscriptionID_options(
            this: Option<Allocated<Self>>,
            record_type: &CKRecordType,
            predicate: &NSPredicate,
            subscription_id: &CKSubscriptionID,
            query_subscription_options: CKQuerySubscriptionOptions,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, a_decoder: &NSCoder)
            -> Id<Self>;

        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Id<CKRecordType>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Id<NSPredicate>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Option<Id<CKRecordZoneID>>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(setZoneID:)]
        pub unsafe fn setZoneID(&self, zone_id: Option<&CKRecordZoneID>);

        #[method(querySubscriptionOptions)]
        pub unsafe fn querySubscriptionOptions(&self) -> CKQuerySubscriptionOptions;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSubscription`
    #[cfg(feature = "CloudKit_CKQuerySubscription")]
    unsafe impl CKQuerySubscription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKRecordZoneSubscription")]
    pub struct CKRecordZoneSubscription;

    #[cfg(feature = "CloudKit_CKRecordZoneSubscription")]
    unsafe impl ClassType for CKRecordZoneSubscription {
        #[inherits(NSObject)]
        type Super = CKSubscription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKRecordZoneSubscription")]
unsafe impl NSCoding for CKRecordZoneSubscription {}

#[cfg(feature = "CloudKit_CKRecordZoneSubscription")]
unsafe impl NSCopying for CKRecordZoneSubscription {}

#[cfg(feature = "CloudKit_CKRecordZoneSubscription")]
unsafe impl NSObjectProtocol for CKRecordZoneSubscription {}

#[cfg(feature = "CloudKit_CKRecordZoneSubscription")]
unsafe impl NSSecureCoding for CKRecordZoneSubscription {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKRecordZoneSubscription")]
    unsafe impl CKRecordZoneSubscription {
        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithZoneID:)]
        pub unsafe fn initWithZoneID(
            this: Option<Allocated<Self>>,
            zone_id: &CKRecordZoneID,
        ) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithZoneID:subscriptionID:)]
        pub unsafe fn initWithZoneID_subscriptionID(
            this: Option<Allocated<Self>>,
            zone_id: &CKRecordZoneID,
            subscription_id: &CKSubscriptionID,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, a_decoder: &NSCoder)
            -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Id<CKRecordZoneID>;

        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Option<Id<CKRecordType>>;

        #[method(setRecordType:)]
        pub unsafe fn setRecordType(&self, record_type: Option<&CKRecordType>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSubscription`
    #[cfg(feature = "CloudKit_CKRecordZoneSubscription")]
    unsafe impl CKRecordZoneSubscription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKDatabaseSubscription")]
    pub struct CKDatabaseSubscription;

    #[cfg(feature = "CloudKit_CKDatabaseSubscription")]
    unsafe impl ClassType for CKDatabaseSubscription {
        #[inherits(NSObject)]
        type Super = CKSubscription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKDatabaseSubscription")]
unsafe impl NSCoding for CKDatabaseSubscription {}

#[cfg(feature = "CloudKit_CKDatabaseSubscription")]
unsafe impl NSCopying for CKDatabaseSubscription {}

#[cfg(feature = "CloudKit_CKDatabaseSubscription")]
unsafe impl NSObjectProtocol for CKDatabaseSubscription {}

#[cfg(feature = "CloudKit_CKDatabaseSubscription")]
unsafe impl NSSecureCoding for CKDatabaseSubscription {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKDatabaseSubscription")]
    unsafe impl CKDatabaseSubscription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithSubscriptionID:)]
        pub unsafe fn initWithSubscriptionID(
            this: Option<Allocated<Self>>,
            subscription_id: &CKSubscriptionID,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, a_decoder: &NSCoder)
            -> Id<Self>;

        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Option<Id<CKRecordType>>;

        #[method(setRecordType:)]
        pub unsafe fn setRecordType(&self, record_type: Option<&CKRecordType>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKNotificationInfo")]
    pub struct CKNotificationInfo;

    #[cfg(feature = "CloudKit_CKNotificationInfo")]
    unsafe impl ClassType for CKNotificationInfo {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKNotificationInfo")]
unsafe impl NSCoding for CKNotificationInfo {}

#[cfg(feature = "CloudKit_CKNotificationInfo")]
unsafe impl NSCopying for CKNotificationInfo {}

#[cfg(feature = "CloudKit_CKNotificationInfo")]
unsafe impl NSObjectProtocol for CKNotificationInfo {}

#[cfg(feature = "CloudKit_CKNotificationInfo")]
unsafe impl NSSecureCoding for CKNotificationInfo {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKNotificationInfo")]
    unsafe impl CKNotificationInfo {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alertBody)]
        pub unsafe fn alertBody(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlertBody:)]
        pub unsafe fn setAlertBody(&self, alert_body: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alertLocalizationKey)]
        pub unsafe fn alertLocalizationKey(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlertLocalizationKey:)]
        pub unsafe fn setAlertLocalizationKey(&self, alert_localization_key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other alertLocalizationArgs)]
        pub unsafe fn alertLocalizationArgs(&self) -> Option<Id<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setAlertLocalizationArgs:)]
        pub unsafe fn setAlertLocalizationArgs(
            &self,
            alert_localization_args: Option<&NSArray<CKRecordFieldKey>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other titleLocalizationKey)]
        pub unsafe fn titleLocalizationKey(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitleLocalizationKey:)]
        pub unsafe fn setTitleLocalizationKey(&self, title_localization_key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other titleLocalizationArgs)]
        pub unsafe fn titleLocalizationArgs(&self) -> Option<Id<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setTitleLocalizationArgs:)]
        pub unsafe fn setTitleLocalizationArgs(
            &self,
            title_localization_args: Option<&NSArray<CKRecordFieldKey>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitleLocalizationKey)]
        pub unsafe fn subtitleLocalizationKey(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubtitleLocalizationKey:)]
        pub unsafe fn setSubtitleLocalizationKey(
            &self,
            subtitle_localization_key: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other subtitleLocalizationArgs)]
        pub unsafe fn subtitleLocalizationArgs(&self) -> Option<Id<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSubtitleLocalizationArgs:)]
        pub unsafe fn setSubtitleLocalizationArgs(
            &self,
            subtitle_localization_args: Option<&NSArray<CKRecordFieldKey>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alertActionLocalizationKey)]
        pub unsafe fn alertActionLocalizationKey(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlertActionLocalizationKey:)]
        pub unsafe fn setAlertActionLocalizationKey(
            &self,
            alert_action_localization_key: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alertLaunchImage)]
        pub unsafe fn alertLaunchImage(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlertLaunchImage:)]
        pub unsafe fn setAlertLaunchImage(&self, alert_launch_image: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other soundName)]
        pub unsafe fn soundName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSoundName:)]
        pub unsafe fn setSoundName(&self, sound_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Id<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);

        #[method(shouldBadge)]
        pub unsafe fn shouldBadge(&self) -> bool;

        #[method(setShouldBadge:)]
        pub unsafe fn setShouldBadge(&self, should_badge: bool);

        #[method(shouldSendContentAvailable)]
        pub unsafe fn shouldSendContentAvailable(&self) -> bool;

        #[method(setShouldSendContentAvailable:)]
        pub unsafe fn setShouldSendContentAvailable(&self, should_send_content_available: bool);

        #[method(shouldSendMutableContent)]
        pub unsafe fn shouldSendMutableContent(&self) -> bool;

        #[method(setShouldSendMutableContent:)]
        pub unsafe fn setShouldSendMutableContent(&self, should_send_mutable_content: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCategory:)]
        pub unsafe fn setCategory(&self, category: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other collapseIDKey)]
        pub unsafe fn collapseIDKey(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCollapseIDKey:)]
        pub unsafe fn setCollapseIDKey(&self, collapse_id_key: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CloudKit_CKNotificationInfo")]
    unsafe impl CKNotificationInfo {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
