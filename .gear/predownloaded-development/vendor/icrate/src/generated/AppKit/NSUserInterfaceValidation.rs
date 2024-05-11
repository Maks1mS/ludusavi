//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSValidatedUserInterfaceItem {
        #[method(action)]
        unsafe fn action(&self) -> Option<Sel>;

        #[method(tag)]
        unsafe fn tag(&self) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn NSValidatedUserInterfaceItem {}
);

extern_protocol!(
    pub unsafe trait NSUserInterfaceValidations {
        #[method(validateUserInterfaceItem:)]
        unsafe fn validateUserInterfaceItem(
            &self,
            item: &ProtocolObject<dyn NSValidatedUserInterfaceItem>,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSUserInterfaceValidations {}
);