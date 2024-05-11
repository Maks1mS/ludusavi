//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSExtensions
    #[cfg(feature = "Foundation_NSFileWrapper")]
    unsafe impl NSFileWrapper {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setIcon:)]
        pub unsafe fn setIcon(&self, icon: Option<&NSImage>);
    }
);