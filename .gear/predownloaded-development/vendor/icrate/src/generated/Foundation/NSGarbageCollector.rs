//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSGarbageCollector")]
    #[deprecated = "Building Garbage Collected apps is no longer supported."]
    pub struct NSGarbageCollector;

    #[cfg(feature = "Foundation_NSGarbageCollector")]
    unsafe impl ClassType for NSGarbageCollector {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSGarbageCollector")]
unsafe impl NSObjectProtocol for NSGarbageCollector {}

extern_methods!(
    #[cfg(feature = "Foundation_NSGarbageCollector")]
    unsafe impl NSGarbageCollector {
        #[method_id(@__retain_semantics Other defaultCollector)]
        pub unsafe fn defaultCollector() -> Id<AnyObject>;

        #[deprecated]
        #[method(isCollecting)]
        pub unsafe fn isCollecting(&self) -> bool;

        #[method(disable)]
        pub unsafe fn disable(&self);

        #[method(enable)]
        pub unsafe fn enable(&self);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(collectIfNeeded)]
        pub unsafe fn collectIfNeeded(&self);

        #[method(collectExhaustively)]
        pub unsafe fn collectExhaustively(&self);

        #[method(disableCollectorForPointer:)]
        pub unsafe fn disableCollectorForPointer(&self, ptr: NonNull<c_void>);

        #[method(enableCollectorForPointer:)]
        pub unsafe fn enableCollectorForPointer(&self, ptr: NonNull<c_void>);

        #[method(zone)]
        pub unsafe fn zone(&self) -> NonNull<NSZone>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSGarbageCollector")]
    unsafe impl NSGarbageCollector {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);