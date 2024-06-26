//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_methods!(
    /// MKMapItem
    #[cfg(feature = "Foundation_NSUserActivity")]
    unsafe impl NSUserActivity {
        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other mapItem)]
        pub unsafe fn mapItem(&self) -> Option<Id<MKMapItem>>;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method(setMapItem:)]
        pub unsafe fn setMapItem(&self, map_item: Option<&MKMapItem>);
    }
);
