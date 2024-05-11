//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSListFormatter")]
    pub struct NSListFormatter;

    #[cfg(feature = "Foundation_NSListFormatter")]
    unsafe impl ClassType for NSListFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSListFormatter")]
unsafe impl NSCoding for NSListFormatter {}

#[cfg(feature = "Foundation_NSListFormatter")]
unsafe impl NSCopying for NSListFormatter {}

#[cfg(feature = "Foundation_NSListFormatter")]
unsafe impl NSObjectProtocol for NSListFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSListFormatter")]
    unsafe impl NSListFormatter {
        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method_id(@__retain_semantics Other itemFormatter)]
        pub unsafe fn itemFormatter(&self) -> Option<Id<NSFormatter>>;

        #[method(setItemFormatter:)]
        pub unsafe fn setItemFormatter(&self, item_formatter: Option<&NSFormatter>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedStringByJoiningStrings:)]
        pub unsafe fn localizedStringByJoiningStrings(strings: &NSArray<NSString>) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromItems:)]
        pub unsafe fn stringFromItems(&self, items: &NSArray) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(&self, obj: Option<&AnyObject>) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSListFormatter")]
    unsafe impl NSListFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
