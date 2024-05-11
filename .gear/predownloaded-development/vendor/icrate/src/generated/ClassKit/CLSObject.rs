//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ClassKit_CLSObject")]
    pub struct CLSObject;

    #[cfg(feature = "ClassKit_CLSObject")]
    unsafe impl ClassType for CLSObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "ClassKit_CLSObject")]
unsafe impl NSCoding for CLSObject {}

#[cfg(feature = "ClassKit_CLSObject")]
unsafe impl NSObjectProtocol for CLSObject {}

#[cfg(feature = "ClassKit_CLSObject")]
unsafe impl NSSecureCoding for CLSObject {}

extern_methods!(
    #[cfg(feature = "ClassKit_CLSObject")]
    unsafe impl CLSObject {
        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateCreated)]
        pub unsafe fn dateCreated(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateLastModified)]
        pub unsafe fn dateLastModified(&self) -> Id<NSDate>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);