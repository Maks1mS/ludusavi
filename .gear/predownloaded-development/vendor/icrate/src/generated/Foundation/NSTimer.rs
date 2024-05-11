//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSTimer")]
    pub struct NSTimer;

    #[cfg(feature = "Foundation_NSTimer")]
    unsafe impl ClassType for NSTimer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSTimer")]
unsafe impl NSObjectProtocol for NSTimer {}

extern_methods!(
    #[cfg(feature = "Foundation_NSTimer")]
    unsafe impl NSTimer {
        #[cfg(feature = "Foundation_NSInvocation")]
        #[method_id(@__retain_semantics Other timerWithTimeInterval:invocation:repeats:)]
        pub unsafe fn timerWithTimeInterval_invocation_repeats(
            ti: NSTimeInterval,
            invocation: &NSInvocation,
            yes_or_no: bool,
        ) -> Id<NSTimer>;

        #[cfg(feature = "Foundation_NSInvocation")]
        #[method_id(@__retain_semantics Other scheduledTimerWithTimeInterval:invocation:repeats:)]
        pub unsafe fn scheduledTimerWithTimeInterval_invocation_repeats(
            ti: NSTimeInterval,
            invocation: &NSInvocation,
            yes_or_no: bool,
        ) -> Id<NSTimer>;

        #[method_id(@__retain_semantics Other timerWithTimeInterval:target:selector:userInfo:repeats:)]
        pub unsafe fn timerWithTimeInterval_target_selector_userInfo_repeats(
            ti: NSTimeInterval,
            a_target: &AnyObject,
            a_selector: Sel,
            user_info: Option<&AnyObject>,
            yes_or_no: bool,
        ) -> Id<NSTimer>;

        #[method_id(@__retain_semantics Other scheduledTimerWithTimeInterval:target:selector:userInfo:repeats:)]
        pub unsafe fn scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
            ti: NSTimeInterval,
            a_target: &AnyObject,
            a_selector: Sel,
            user_info: Option<&AnyObject>,
            yes_or_no: bool,
        ) -> Id<NSTimer>;

        #[method_id(@__retain_semantics Other timerWithTimeInterval:repeats:block:)]
        pub unsafe fn timerWithTimeInterval_repeats_block(
            interval: NSTimeInterval,
            repeats: bool,
            block: &Block<(NonNull<NSTimer>,), ()>,
        ) -> Id<NSTimer>;

        #[method_id(@__retain_semantics Other scheduledTimerWithTimeInterval:repeats:block:)]
        pub unsafe fn scheduledTimerWithTimeInterval_repeats_block(
            interval: NSTimeInterval,
            repeats: bool,
            block: &Block<(NonNull<NSTimer>,), ()>,
        ) -> Id<NSTimer>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithFireDate:interval:repeats:block:)]
        pub unsafe fn initWithFireDate_interval_repeats_block(
            this: Option<Allocated<Self>>,
            date: &NSDate,
            interval: NSTimeInterval,
            repeats: bool,
            block: &Block<(NonNull<NSTimer>,), ()>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithFireDate:interval:target:selector:userInfo:repeats:)]
        pub unsafe fn initWithFireDate_interval_target_selector_userInfo_repeats(
            this: Option<Allocated<Self>>,
            date: &NSDate,
            ti: NSTimeInterval,
            t: &AnyObject,
            s: Sel,
            ui: Option<&AnyObject>,
            rep: bool,
        ) -> Id<Self>;

        #[method(fire)]
        pub unsafe fn fire(&self);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other fireDate)]
        pub unsafe fn fireDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setFireDate:)]
        pub unsafe fn setFireDate(&self, fire_date: &NSDate);

        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[method(tolerance)]
        pub unsafe fn tolerance(&self) -> NSTimeInterval;

        #[method(setTolerance:)]
        pub unsafe fn setTolerance(&self, tolerance: NSTimeInterval);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSTimer")]
    unsafe impl NSTimer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);