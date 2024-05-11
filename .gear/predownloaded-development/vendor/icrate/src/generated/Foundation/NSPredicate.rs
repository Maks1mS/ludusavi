//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPredicate")]
    pub struct NSPredicate;

    #[cfg(feature = "Foundation_NSPredicate")]
    unsafe impl ClassType for NSPredicate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSPredicate")]
unsafe impl NSCoding for NSPredicate {}

#[cfg(feature = "Foundation_NSPredicate")]
unsafe impl NSCopying for NSPredicate {}

#[cfg(feature = "Foundation_NSPredicate")]
unsafe impl NSObjectProtocol for NSPredicate {}

#[cfg(feature = "Foundation_NSPredicate")]
unsafe impl NSSecureCoding for NSPredicate {}

extern_methods!(
    #[cfg(feature = "Foundation_NSPredicate")]
    unsafe impl NSPredicate {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateWithFormat:argumentArray:)]
        pub unsafe fn predicateWithFormat_argumentArray(
            predicate_format: &NSString,
            arguments: Option<&NSArray>,
        ) -> Id<NSPredicate>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other predicateFromMetadataQueryString:)]
        pub unsafe fn predicateFromMetadataQueryString(
            query_string: &NSString,
        ) -> Option<Id<NSPredicate>>;

        #[method_id(@__retain_semantics Other predicateWithValue:)]
        pub unsafe fn predicateWithValue(value: bool) -> Id<NSPredicate>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateWithBlock:)]
        pub unsafe fn predicateWithBlock(
            block: &Block<(*mut AnyObject, *mut NSDictionary<NSString, AnyObject>), Bool>,
        ) -> Id<NSPredicate>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other predicateFormat)]
        pub unsafe fn predicateFormat(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateWithSubstitutionVariables:)]
        pub unsafe fn predicateWithSubstitutionVariables(
            &self,
            variables: &NSDictionary<NSString, AnyObject>,
        ) -> Id<Self>;

        #[method(evaluateWithObject:)]
        pub unsafe fn evaluateWithObject(&self, object: Option<&AnyObject>) -> bool;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(evaluateWithObject:substitutionVariables:)]
        pub unsafe fn evaluateWithObject_substitutionVariables(
            &self,
            object: Option<&AnyObject>,
            bindings: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> bool;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSPredicate")]
    unsafe impl NSPredicate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other filteredArrayUsingPredicate:)]
        pub unsafe fn filteredArrayUsingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Id<NSArray<ObjectType>>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&mut self, predicate: &NSPredicate);
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSSet")]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other filteredSetUsingPredicate:)]
        pub unsafe fn filteredSetUsingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Id<NSSet<ObjectType>>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSMutableSet")]
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&mut self, predicate: &NSPredicate);
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other filteredOrderedSetUsingPredicate:)]
        pub unsafe fn filteredOrderedSetUsingPredicate(
            &self,
            p: &NSPredicate,
        ) -> Id<NSOrderedSet<ObjectType>>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&mut self, p: &NSPredicate);
    }
);