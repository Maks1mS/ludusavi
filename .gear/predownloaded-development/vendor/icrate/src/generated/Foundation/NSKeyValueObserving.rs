//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSKeyValueObservingOptions {
        NSKeyValueObservingOptionNew = 0x01,
        NSKeyValueObservingOptionOld = 0x02,
        NSKeyValueObservingOptionInitial = 0x04,
        NSKeyValueObservingOptionPrior = 0x08,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSKeyValueChange {
        NSKeyValueChangeSetting = 1,
        NSKeyValueChangeInsertion = 2,
        NSKeyValueChangeRemoval = 3,
        NSKeyValueChangeReplacement = 4,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSKeyValueSetMutationKind {
        NSKeyValueUnionSetMutation = 1,
        NSKeyValueMinusSetMutation = 2,
        NSKeyValueIntersectSetMutation = 3,
        NSKeyValueSetSetMutation = 4,
    }
);

typed_enum!(
    pub type NSKeyValueChangeKey = NSString;
);

extern_static!(NSKeyValueChangeKindKey: &'static NSKeyValueChangeKey);

extern_static!(NSKeyValueChangeNewKey: &'static NSKeyValueChangeKey);

extern_static!(NSKeyValueChangeOldKey: &'static NSKeyValueChangeKey);

extern_static!(NSKeyValueChangeIndexesKey: &'static NSKeyValueChangeKey);

extern_static!(NSKeyValueChangeNotificationIsPriorKey: &'static NSKeyValueChangeKey);

extern_methods!(
    /// NSKeyValueObserverRegistration
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[cfg(all(feature = "Foundation_NSIndexSet", feature = "Foundation_NSString"))]
        #[method(addObserver:toObjectsAtIndexes:forKeyPath:options:context:)]
        pub unsafe fn addObserver_toObjectsAtIndexes_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            indexes: &NSIndexSet,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(all(feature = "Foundation_NSIndexSet", feature = "Foundation_NSString"))]
        #[method(removeObserver:fromObjectsAtIndexes:forKeyPath:context:)]
        pub unsafe fn removeObserver_fromObjectsAtIndexes_forKeyPath_context(
            &self,
            observer: &NSObject,
            indexes: &NSIndexSet,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[cfg(all(feature = "Foundation_NSIndexSet", feature = "Foundation_NSString"))]
        #[method(removeObserver:fromObjectsAtIndexes:forKeyPath:)]
        pub unsafe fn removeObserver_fromObjectsAtIndexes_forKeyPath(
            &self,
            observer: &NSObject,
            indexes: &NSIndexSet,
            key_path: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, key_path: &NSString);
    }
);

extern_methods!(
    /// NSKeyValueObserverRegistration
    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[cfg(feature = "Foundation_NSString")]
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, key_path: &NSString);
    }
);

extern_methods!(
    /// NSKeyValueObserverRegistration
    #[cfg(feature = "Foundation_NSSet")]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[cfg(feature = "Foundation_NSString")]
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, key_path: &NSString);
    }
);