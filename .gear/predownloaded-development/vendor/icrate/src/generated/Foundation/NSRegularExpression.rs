//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSRegularExpressionOptions {
        NSRegularExpressionCaseInsensitive = 1 << 0,
        NSRegularExpressionAllowCommentsAndWhitespace = 1 << 1,
        NSRegularExpressionIgnoreMetacharacters = 1 << 2,
        NSRegularExpressionDotMatchesLineSeparators = 1 << 3,
        NSRegularExpressionAnchorsMatchLines = 1 << 4,
        NSRegularExpressionUseUnixLineSeparators = 1 << 5,
        NSRegularExpressionUseUnicodeWordBoundaries = 1 << 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSRegularExpression")]
    pub struct NSRegularExpression;

    #[cfg(feature = "Foundation_NSRegularExpression")]
    unsafe impl ClassType for NSRegularExpression {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSRegularExpression")]
unsafe impl NSCoding for NSRegularExpression {}

#[cfg(feature = "Foundation_NSRegularExpression")]
unsafe impl NSCopying for NSRegularExpression {}

#[cfg(feature = "Foundation_NSRegularExpression")]
unsafe impl NSObjectProtocol for NSRegularExpression {}

#[cfg(feature = "Foundation_NSRegularExpression")]
unsafe impl NSSecureCoding for NSRegularExpression {}

extern_methods!(
    #[cfg(feature = "Foundation_NSRegularExpression")]
    unsafe impl NSRegularExpression {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other regularExpressionWithPattern:options:error:_)]
        pub unsafe fn regularExpressionWithPattern_options_error(
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Id<NSRegularExpression>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithPattern:options:error:_)]
        pub unsafe fn initWithPattern_options_error(
            this: Option<Allocated<Self>>,
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pattern)]
        pub unsafe fn pattern(&self) -> Id<NSString>;

        #[method(options)]
        pub unsafe fn options(&self) -> NSRegularExpressionOptions;

        #[method(numberOfCaptureGroups)]
        pub unsafe fn numberOfCaptureGroups(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other escapedPatternForString:)]
        pub unsafe fn escapedPatternForString(string: &NSString) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSRegularExpression")]
    unsafe impl NSRegularExpression {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMatchingOptions {
        NSMatchingReportProgress = 1 << 0,
        NSMatchingReportCompletion = 1 << 1,
        NSMatchingAnchored = 1 << 2,
        NSMatchingWithTransparentBounds = 1 << 3,
        NSMatchingWithoutAnchoringBounds = 1 << 4,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMatchingFlags {
        NSMatchingProgress = 1 << 0,
        NSMatchingCompleted = 1 << 1,
        NSMatchingHitEnd = 1 << 2,
        NSMatchingRequiredEnd = 1 << 3,
        NSMatchingInternalError = 1 << 4,
    }
);

extern_methods!(
    /// NSMatching
    #[cfg(feature = "Foundation_NSRegularExpression")]
    unsafe impl NSRegularExpression {
        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method(enumerateMatchesInString:options:range:usingBlock:)]
        pub unsafe fn enumerateMatchesInString_options_range_usingBlock(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
            block: &Block<(*mut NSTextCheckingResult, NSMatchingFlags, NonNull<Bool>), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method_id(@__retain_semantics Other matchesInString:options:range:)]
        pub unsafe fn matchesInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> Id<NSArray<NSTextCheckingResult>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(numberOfMatchesInString:options:range:)]
        pub unsafe fn numberOfMatchesInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> NSUInteger;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method_id(@__retain_semantics Other firstMatchInString:options:range:)]
        pub unsafe fn firstMatchInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> Option<Id<NSTextCheckingResult>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(rangeOfFirstMatchInString:options:range:)]
        pub unsafe fn rangeOfFirstMatchInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> NSRange;
    }
);

extern_methods!(
    /// NSReplacement
    #[cfg(feature = "Foundation_NSRegularExpression")]
    unsafe impl NSRegularExpression {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringByReplacingMatchesInString:options:range:withTemplate:)]
        pub unsafe fn stringByReplacingMatchesInString_options_range_withTemplate(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
            templ: &NSString,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSMutableString",
            feature = "Foundation_NSString"
        ))]
        #[method(replaceMatchesInString:options:range:withTemplate:)]
        pub unsafe fn replaceMatchesInString_options_range_withTemplate(
            &self,
            string: &NSMutableString,
            options: NSMatchingOptions,
            range: NSRange,
            templ: &NSString,
        ) -> NSUInteger;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method_id(@__retain_semantics Other replacementStringForResult:inString:offset:template:)]
        pub unsafe fn replacementStringForResult_inString_offset_template(
            &self,
            result: &NSTextCheckingResult,
            string: &NSString,
            offset: NSInteger,
            templ: &NSString,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other escapedTemplateForString:)]
        pub unsafe fn escapedTemplateForString(string: &NSString) -> Id<NSString>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDataDetector")]
    pub struct NSDataDetector;

    #[cfg(feature = "Foundation_NSDataDetector")]
    unsafe impl ClassType for NSDataDetector {
        #[inherits(NSObject)]
        type Super = NSRegularExpression;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSDataDetector")]
unsafe impl NSCoding for NSDataDetector {}

#[cfg(feature = "Foundation_NSDataDetector")]
unsafe impl NSCopying for NSDataDetector {}

#[cfg(feature = "Foundation_NSDataDetector")]
unsafe impl NSObjectProtocol for NSDataDetector {}

#[cfg(feature = "Foundation_NSDataDetector")]
unsafe impl NSSecureCoding for NSDataDetector {}

extern_methods!(
    #[cfg(feature = "Foundation_NSDataDetector")]
    unsafe impl NSDataDetector {
        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other dataDetectorWithTypes:error:_)]
        pub unsafe fn dataDetectorWithTypes_error(
            checking_types: NSTextCheckingTypes,
        ) -> Result<Id<NSDataDetector>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Init initWithTypes:error:_)]
        pub unsafe fn initWithTypes_error(
            this: Option<Allocated<Self>>,
            checking_types: NSTextCheckingTypes,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method(checkingTypes)]
        pub unsafe fn checkingTypes(&self) -> NSTextCheckingTypes;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSRegularExpression`
    #[cfg(feature = "Foundation_NSDataDetector")]
    unsafe impl NSDataDetector {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithPattern:options:error:_)]
        pub unsafe fn initWithPattern_options_error(
            this: Option<Allocated<Self>>,
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSDataDetector")]
    unsafe impl NSDataDetector {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);