//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSXMLNodeKind {
        NSXMLInvalidKind = 0,
        NSXMLDocumentKind = 1,
        NSXMLElementKind = 2,
        NSXMLAttributeKind = 3,
        NSXMLNamespaceKind = 4,
        NSXMLProcessingInstructionKind = 5,
        NSXMLCommentKind = 6,
        NSXMLTextKind = 7,
        NSXMLDTDKind = 8,
        NSXMLEntityDeclarationKind = 9,
        NSXMLAttributeDeclarationKind = 10,
        NSXMLElementDeclarationKind = 11,
        NSXMLNotationDeclarationKind = 12,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXMLNode")]
    pub struct NSXMLNode;

    #[cfg(feature = "Foundation_NSXMLNode")]
    unsafe impl ClassType for NSXMLNode {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSXMLNode")]
unsafe impl NSCopying for NSXMLNode {}

#[cfg(feature = "Foundation_NSXMLNode")]
unsafe impl NSObjectProtocol for NSXMLNode {}

extern_methods!(
    #[cfg(feature = "Foundation_NSXMLNode")]
    unsafe impl NSXMLNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(this: Option<Allocated<Self>>, kind: NSXMLNodeKind) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other document)]
        pub unsafe fn document() -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSXMLElement")]
        #[method_id(@__retain_semantics Other documentWithRootElement:)]
        pub unsafe fn documentWithRootElement(element: &NSXMLElement) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other elementWithName:)]
        pub unsafe fn elementWithName(name: &NSString) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other elementWithName:URI:)]
        pub unsafe fn elementWithName_URI(name: &NSString, uri: &NSString) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other elementWithName:stringValue:)]
        pub unsafe fn elementWithName_stringValue(
            name: &NSString,
            string: &NSString,
        ) -> Id<AnyObject>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other elementWithName:children:attributes:)]
        pub unsafe fn elementWithName_children_attributes(
            name: &NSString,
            children: Option<&NSArray<NSXMLNode>>,
            attributes: Option<&NSArray<NSXMLNode>>,
        ) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other attributeWithName:stringValue:)]
        pub unsafe fn attributeWithName_stringValue(
            name: &NSString,
            string_value: &NSString,
        ) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other attributeWithName:URI:stringValue:)]
        pub unsafe fn attributeWithName_URI_stringValue(
            name: &NSString,
            uri: &NSString,
            string_value: &NSString,
        ) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other namespaceWithName:stringValue:)]
        pub unsafe fn namespaceWithName_stringValue(
            name: &NSString,
            string_value: &NSString,
        ) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other processingInstructionWithName:stringValue:)]
        pub unsafe fn processingInstructionWithName_stringValue(
            name: &NSString,
            string_value: &NSString,
        ) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other commentWithStringValue:)]
        pub unsafe fn commentWithStringValue(string_value: &NSString) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textWithStringValue:)]
        pub unsafe fn textWithStringValue(string_value: &NSString) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other DTDNodeWithXMLString:)]
        pub unsafe fn DTDNodeWithXMLString(string: &NSString) -> Option<Id<AnyObject>>;

        #[method(kind)]
        pub unsafe fn kind(&self) -> NSXMLNodeKind;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<AnyObject>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, object_value: Option<&AnyObject>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setStringValue:)]
        pub unsafe fn setStringValue(&self, string_value: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setStringValue:resolvingEntities:)]
        pub unsafe fn setStringValue_resolvingEntities(&self, string: &NSString, resolve: bool);

        #[method(index)]
        pub unsafe fn index(&self) -> NSUInteger;

        #[method(level)]
        pub unsafe fn level(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSXMLDocument")]
        #[method_id(@__retain_semantics Other rootDocument)]
        pub unsafe fn rootDocument(&self) -> Option<Id<NSXMLDocument>>;

        #[method_id(@__retain_semantics Other parent)]
        pub unsafe fn parent(&self) -> Option<Id<NSXMLNode>>;

        #[method(childCount)]
        pub unsafe fn childCount(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other children)]
        pub unsafe fn children(&self) -> Option<Id<NSArray<NSXMLNode>>>;

        #[method_id(@__retain_semantics Other childAtIndex:)]
        pub unsafe fn childAtIndex(&self, index: NSUInteger) -> Option<Id<NSXMLNode>>;

        #[method_id(@__retain_semantics Other previousSibling)]
        pub unsafe fn previousSibling(&self) -> Option<Id<NSXMLNode>>;

        #[method_id(@__retain_semantics Other nextSibling)]
        pub unsafe fn nextSibling(&self) -> Option<Id<NSXMLNode>>;

        #[method_id(@__retain_semantics Other previousNode)]
        pub unsafe fn previousNode(&self) -> Option<Id<NSXMLNode>>;

        #[method_id(@__retain_semantics Other nextNode)]
        pub unsafe fn nextNode(&self) -> Option<Id<NSXMLNode>>;

        #[method(detach)]
        pub unsafe fn detach(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other XPath)]
        pub unsafe fn XPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localName)]
        pub unsafe fn localName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other prefix)]
        pub unsafe fn prefix(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other URI)]
        pub unsafe fn URI(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setURI:)]
        pub unsafe fn setURI(&self, uri: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localNameForName:)]
        pub unsafe fn localNameForName(name: &NSString) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other prefixForName:)]
        pub unsafe fn prefixForName(name: &NSString) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other predefinedNamespaceForPrefix:)]
        pub unsafe fn predefinedNamespaceForPrefix(name: &NSString) -> Option<Id<NSXMLNode>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other XMLString)]
        pub unsafe fn XMLString(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other XMLStringWithOptions:)]
        pub unsafe fn XMLStringWithOptions(&self, options: NSXMLNodeOptions) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other canonicalXMLStringPreservingComments:)]
        pub unsafe fn canonicalXMLStringPreservingComments(&self, comments: bool) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other nodesForXPath:error:_)]
        pub unsafe fn nodesForXPath_error(
            &self,
            xpath: &NSString,
        ) -> Result<Id<NSArray<NSXMLNode>>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other objectsForXQuery:constants:error:_)]
        pub unsafe fn objectsForXQuery_constants_error(
            &self,
            xquery: &NSString,
            constants: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Id<NSArray>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other objectsForXQuery:error:_)]
        pub unsafe fn objectsForXQuery_error(
            &self,
            xquery: &NSString,
        ) -> Result<Id<NSArray>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSXMLNode")]
    unsafe impl NSXMLNode {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);