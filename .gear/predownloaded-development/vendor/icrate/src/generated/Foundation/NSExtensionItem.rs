//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSExtensionItem")]
    pub struct NSExtensionItem;

    #[cfg(feature = "Foundation_NSExtensionItem")]
    unsafe impl ClassType for NSExtensionItem {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSExtensionItem")]
unsafe impl NSCoding for NSExtensionItem {}

#[cfg(feature = "Foundation_NSExtensionItem")]
unsafe impl NSCopying for NSExtensionItem {}

#[cfg(feature = "Foundation_NSExtensionItem")]
unsafe impl NSObjectProtocol for NSExtensionItem {}

#[cfg(feature = "Foundation_NSExtensionItem")]
unsafe impl NSSecureCoding for NSExtensionItem {}

extern_methods!(
    #[cfg(feature = "Foundation_NSExtensionItem")]
    unsafe impl NSExtensionItem {
        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedContentText)]
        pub unsafe fn attributedContentText(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedContentText:)]
        pub unsafe fn setAttributedContentText(
            &self,
            attributed_content_text: Option<&NSAttributedString>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSItemProvider"))]
        #[method_id(@__retain_semantics Other attachments)]
        pub unsafe fn attachments(&self) -> Option<Id<NSArray<NSItemProvider>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSItemProvider"))]
        #[method(setAttachments:)]
        pub unsafe fn setAttachments(&self, attachments: Option<&NSArray<NSItemProvider>>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSExtensionItem")]
    unsafe impl NSExtensionItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSExtensionItemAttributedTitleKey: Option<&'static NSString>);

extern_static!(NSExtensionItemAttributedContentTextKey: Option<&'static NSString>);

extern_static!(NSExtensionItemAttachmentsKey: Option<&'static NSString>);